use super::parser::{self, Error, Rule};
use anyhow::{anyhow, Context, Result};
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub type Names = HashMap<String, String>;
pub type Children = Vec<Child>;

#[derive(Debug, PartialEq)]
pub enum Child {
    AtRule {
        name: Option<String>,
        rule: Option<String>,
        children: Children,
    },
    Comment {
        value: Option<String>,
    },
    Property {
        name: Option<String>,
        value: Option<String>,
    },
    SelectRule {
        rule: Option<String>,
        children: Children,
    },
}

impl fmt::Display for Child {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Child::AtRule {
                name,
                rule,
                children,
            } => {
                if let (Some(name), Some(rule)) = (name, rule) {
                    if children.is_empty() {
                        write!(formatter, "@{} {}; ", name, rule.trim())?;
                    } else {
                        write!(formatter, "@{} {} {{ ", name, rule.trim())?;

                        for child in children {
                            write!(formatter, "{}", child)?;
                        }

                        writeln!(formatter, "}}")?;
                    }
                } else if let Some(name) = name {
                    if children.is_empty() {
                        write!(formatter, "@{};", name)?;
                    } else {
                        write!(formatter, "@{} {{ ", name)?;

                        for child in children {
                            write!(formatter, "{}", child)?;
                        }

                        writeln!(formatter, "}}")?;
                    }
                }
            }
            Child::SelectRule { rule, children } => {
                if children.is_empty() {
                    write!(formatter, "")?;
                } else if let Some(rule) = rule {
                    write!(formatter, "{} {{ ", rule)?;

                    for child in children {
                        write!(formatter, "{}", child)?;
                    }

                    writeln!(formatter, "}}")?;
                }
            }
            Child::Property { name, value } => {
                if let (Some(name), Some(value)) = (name, value) {
                    write!(formatter, "{}: {}; ", name, value)?;
                } else if let Some(name) = name {
                    write!(formatter, "{}:; ", name)?;
                }
            }
            Child::Comment { value } => {
                if let Some(value) = value {
                    write!(formatter, "{}", value)?;
                }
            }
        };

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParserContext<'c> {
    pub module: &'c mut Module,
    pub name: &'c str,
    pub absolute_path: &'c PathBuf,
    pub stylesheet: &'c mut Stylesheet,
}

impl<'c> ParserContext<'c> {
    fn add_name(&mut self, name: String) -> String {
        let res = self.module.names.entry(name.clone()).or_insert(format!(
            "{}__{}__{}",
            &self.name, &name, &self.stylesheet.names_count
        ));

        self.stylesheet.names_count += 1;

        res.to_owned()
    }
}

#[derive(Debug, PartialEq)]
pub struct Module {
    pub children: Children,
    pub names: Names,
    pub file_path: PathBuf,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
impl Default for Module {
    fn default() -> Self {
        use std::str::FromStr;

        let path = PathBuf::from_str(file!()).unwrap();

        Self {
            children: Vec::new(),
            names: HashMap::new(),
            file_path: path,
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // write!(formatter, "{}", value)
        for child in &self.children {
            write!(formatter, "{}", child)?;
        }

        Ok(())
    }
}

impl<'m> Module {
    pub fn new(stylesheet: &mut Stylesheet, file_path: PathBuf, input: &'m str) -> Result<Self> {
        let pairs = parser::stylesheet(input)?;
        let mut module = Module {
            children: Children::new(),
            names: Names::new(),
            file_path: file_path.clone(),
        };
        let mut context = ParserContext {
            module: &mut module,
            name: file_path
                .file_stem()
                .context("No file stem")?
                .to_str()
                .context("Invalid file path")?,
            absolute_path: &file_path
                .parent()
                .context("No parent directory")?
                .to_path_buf(),
            stylesheet,
        };

        for pair in pairs {
            let child = match pair.as_rule() {
                Rule::comment => comment(pair),
                Rule::atrule => atrule(&mut context, pair)?,
                Rule::selectrule => selectrule(&mut context, pair)?,
                Rule::EOI => None,
                _ => return Err(anyhow!(Error::from(pair))),
            };

            if let Some(child) = child {
                context.module.children.push(child);
            }
        }

        Ok(module)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Stylesheet {
    pub modules: HashMap<PathBuf, Module>,
    pub names_count: u64,
}

impl Stylesheet {
    pub fn add_module(&mut self, module_path: &Path) -> Result<&Module> {
        let mut file = File::open(module_path)?;
        let mut input = String::new();

        file.read_to_string(&mut input)?;

        let module = Module::new(self, module_path.to_path_buf(), &input)?;

        Ok(self
            .modules
            .entry(module_path.to_path_buf())
            .or_insert(module))
    }

    #[cfg(test)]
    #[allow(clippy::unwrap_used)]
    fn add_test_module(&mut self, input: &str) -> Result<&Module> {
        use std::str::FromStr;

        let path = PathBuf::from_str(file!()).unwrap();
        let module = Module::new(self, path.clone(), input)?;

        Ok(self.modules.entry(path).or_insert(module))
    }
}

pub fn atrule<'t>(context: &mut ParserContext, pair: Pair<'t, Rule>) -> Result<Option<Child>> {
    let mut name: Option<String> = None;
    let mut rule: Option<String> = None;
    let mut children = Vec::new();

    for pair in pair.into_inner() {
        let child = match pair.as_rule() {
            Rule::identifier => {
                name = Some(pair.as_str().into());
                None
            }
            Rule::atrule_rule => {
                if Some("keyframes".into()) == name {
                    rule = Some((&context.add_name(pair.as_str().trim().into())).to_string());
                } else if Some("import".into()) == name {
                    let quotes: &[_] = &['"', '\''];
                    let path = context
                        .absolute_path
                        .clone()
                        .join(pair.as_str().trim_matches(quotes));
                    let import = context.stylesheet.add_module(&path)?;

                    for (old, new) in import.names.iter() {
                        context
                            .module
                            .names
                            .entry(old.clone())
                            .or_insert_with(|| new.clone());
                    }

                    return Ok(None);
                } else {
                    rule = Some(pair.as_str().into());
                }

                None
            }
            Rule::comment | Rule::line_comment => comment(pair),
            Rule::property => property(context, pair)?,
            Rule::atrule => atrule(context, pair)?,
            Rule::selectrule => selectrule(context, pair)?,
            _ => return Err(anyhow!(Error::from(pair))),
        };

        if let Some(child) = child {
            children.push(child);
        }
    }

    Ok(Some(Child::AtRule {
        name,
        rule,
        children,
    }))
}

pub fn comment(pair: Pair<Rule>) -> Option<Child> {
    Some(Child::Comment {
        value: Some(pair.as_str().into()),
    })
}

pub fn property<'t>(context: &mut ParserContext, pair: Pair<'t, Rule>) -> Result<Option<Child>> {
    let mut name: Option<String> = None;
    let mut value: Option<String> = None;

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                name = Some(pair.as_str().into());
            }
            Rule::property_value => {
                if Some("animation".into()) == name || Some("animation-name".into()) == name {
                    value = replace_names(context, parser::animation(pair.as_str())?)?;
                } else {
                    value = Some(pair.as_str().trim().into());
                }
            }
            _ => return Err(anyhow!(Error::from(pair))),
        }
    }

    Ok(Some(Child::Property { name, value }))
}

pub fn selectrule<'t>(context: &mut ParserContext, pair: Pair<'t, Rule>) -> Result<Option<Child>> {
    let mut rule: Option<String> = None;
    let mut children = Vec::new();

    for pair in pair.into_inner() {
        let child = match pair.as_rule() {
            Rule::selectrule_rule => {
                rule = replace_names(context, parser::selector(pair.as_str())?)?;

                None
            }
            Rule::comment | Rule::line_comment => comment(pair),
            Rule::property => property(context, pair)?,
            Rule::atrule => atrule(context, pair)?,
            Rule::selectrule => selectrule(context, pair)?,
            _ => return Err(anyhow!(Error::from(pair))),
        };

        if let Some(child) = child {
            children.push(child);
        }
    }

    Ok(Some(Child::SelectRule { rule, children }))
}

pub fn replace_names(context: &mut ParserContext, pairs: Pairs<Rule>) -> Result<Option<String>> {
    let mut result = String::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::identifier => {
                result.push_str(&context.add_name(pair.as_str().trim().into()));
            }
            Rule::selector_class => {
                result.push_str(&format!(
                    ".{}",
                    &context.add_name(pair.as_str()[1..].trim().into())
                ));
            }
            _ => {
                result.push_str(pair.as_str());
            }
        }
    }

    result = result.trim().into();

    if result.is_empty() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn parses_empty_stylesheet() {
        assert_eq!(
            Stylesheet::default(),
            Stylesheet {
                names_count: 0,
                modules: HashMap::new(),
            }
        )
    }

    #[test]
    fn parses_empty_select_rule() {
        assert_eq!(
            Stylesheet::default().add_test_module(".foobar {}").unwrap(),
            &Module {
                children: vec![Child::SelectRule {
                    children: Vec::new(),
                    rule: Some(".ast__foobar__0".into()),
                }],
                names: vec![("foobar".into(), "ast__foobar__0".into())]
                    .into_iter()
                    .collect(),
                ..Module::default()
            }
        )
    }

    #[test]
    fn parses_select_rule_with_property() {
        assert_eq!(
            Stylesheet::default()
                .add_test_module(".foobar { color: red; }")
                .unwrap(),
            &Module {
                children: vec![Child::SelectRule {
                    rule: Some(".ast__foobar__0".into()),
                    children: vec![Child::Property {
                        name: Some("color".into()),
                        value: Some("red".into())
                    }],
                }],
                names: vec![("foobar".into(), "ast__foobar__0".into())]
                    .into_iter()
                    .collect(),
                ..Module::default()
            }
        )
    }

    #[test]
    fn parses_empty_at_rule() {
        assert_eq!(
            Stylesheet::default()
                .add_test_module("@keyframes foobar;")
                .unwrap(),
            &Module {
                children: vec![Child::AtRule {
                    name: Some("keyframes".into()),
                    children: Vec::new(),
                    rule: Some("ast__foobar__0".into()),
                }],
                names: vec![("foobar".into(), "ast__foobar__0".into())]
                    .into_iter()
                    .collect(),
                ..Module::default()
            }
        )
    }

    #[test]
    fn format_empty_module() {
        let mut stylesheet = Stylesheet::default();
        let module = stylesheet.add_test_module("").unwrap();

        assert_eq!(format!("{}", module), String::new());
    }

    #[test]
    fn format_select_rule() {
        let mut stylesheet = Stylesheet::default();
        let module = stylesheet
            .add_test_module("p.foobar  {  color :  #fff ;  }")
            .unwrap();

        assert_eq!(
            &format!("{}", module),
            "p.ast__foobar__0 { color: #fff; }\n"
        );
    }

    #[test]
    fn format_at_rule() {
        let mut stylesheet = Stylesheet::default();
        let module = stylesheet
            .add_test_module("@keyframes animation {0% { top: 0;} 100% {top: 100px;}}")
            .unwrap();

        assert_eq!(
            &format!("{}", module),
            "@keyframes ast__animation__0 { 0% { top: 0; }\n100% { top: 100px; }\n}\n"
        );
    }

    #[test]
    fn error_is_unclosed_block() {
        assert!(Stylesheet::default().add_test_module("p {").is_err());
    }

    #[test]
    fn error_is_property_without_value() {
        assert!(Stylesheet::default().add_test_module("p {color;}").is_err());
    }
}
