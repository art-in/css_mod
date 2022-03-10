use once_cell::sync::OnceCell;
use std::{collections::HashMap, ops::Index, panic, path::PathBuf};

/// Mappings container for all CSS modules.
#[derive(Default, Debug)]
pub struct Mappings<'ms>(pub HashMap<&'ms str, Mapping<'ms>>);

impl<'ms> Mappings<'ms> {
    pub fn add_mapping(
        mut self,
        css_module_path: &'ms str,
        names: impl IntoIterator<Item = (&'ms str, &'ms str)>,
    ) -> Self {
        self.0.insert(
            css_module_path,
            Mapping {
                names: names.into_iter().collect(),
                css_module_path,
            },
        );

        self
    }
}

/// Mapping between original local names and transformed global names for particular CSS module.
#[derive(Default, Clone, Debug)]
pub struct Mapping<'m> {
    names: HashMap<&'m str, &'m str>,
    css_module_path: &'m str,
}

impl<'m> Index<&str> for Mapping<'m> {
    type Output = &'m str;

    fn index<'i>(&self, local_name: &'i str) -> &&'m str {
        self.names.get(local_name).unwrap_or_else(|| {
            panic!(
                r#"Name "{}" was not found in {:?}"#,
                local_name, self.css_module_path
            )
        })
    }
}

pub static MAPPINGS: OnceCell<Mappings> = OnceCell::new();

pub fn get_mapping<'g>(css_module_path: PathBuf) -> &'g Mapping<'g> {
    MAPPINGS
        .get()
        .expect(
            "Mappings are not initialized. \
                Help: call css_mod::init!() once early (eg. in main.rs)",
        )
        .0
        .get(css_module_path.to_str().unwrap())
        .unwrap_or_else(|| panic!("CSS module was not found: {:?}", css_module_path))
}
