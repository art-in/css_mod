use crate::utils::resolve_module_file_path;
use once_cell::sync::OnceCell;
use std::{collections::HashMap, ops::Index, panic};

/// Mapping of original local names to transformed global names for particular CSS module.
#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
pub struct Mappings<'ms> {
    map: HashMap<&'ms str, Mapping<'ms>>,
    is_windows_host: bool,
}

impl<'ms> Mappings<'ms> {
    pub fn is_windows_host(mut self, val: bool) -> Self {
        self.is_windows_host = val;
        self
    }

    pub fn add_mapping(
        mut self,
        css_module_path: &'ms str,
        names: impl IntoIterator<Item = (&'ms str, &'ms str)>,
    ) -> Self {
        self.map.insert(
            css_module_path,
            Mapping {
                names: names.into_iter().collect(),
                css_module_path,
            },
        );
        self
    }
}

pub static MAPPINGS: OnceCell<Mappings> = OnceCell::new();

pub fn get_mapping<'g>(source_path: &str, css_module_path: &str) -> &'g Mapping<'g> {
    let mappings = MAPPINGS.get().expect(
        "Mappings are not initialized. Help: call css_mod::init!() once early (eg. in main.rs)",
    );

    // TODO: resolve CSS module paths when compiling, not on performance critical runtime path.
    // try to move it to proc_macro when `proc_macro::Span` is stabilized
    // https://github.com/rust-lang/rust/issues/54725
    let module_file_path =
        resolve_module_file_path(source_path, css_module_path, mappings.is_windows_host);

    mappings
        .map
        .get(&module_file_path as &str)
        .unwrap_or_else(|| panic!("CSS module was not found: {:?}", module_file_path))
}
