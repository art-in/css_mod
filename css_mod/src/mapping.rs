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

// Gets CSS name mapping
//
// This is private API and intended to be used through public `css_mod::get!()` macro only.
//
// * `source_path`: Source code file from which function is called. It is expected to be
//                  result of `file!()` macro, which returns path in host-os-style
// * `css_module_path`: CSS module file path. Relative to source file - cannot be absolute.
//                      In posix-style - cannot be in windows style (ie. with backward slashes)
pub fn get_mapping<'g>(source_path: &str, css_module_path: &str) -> &'g Mapping<'g> {
    let mappings = MAPPINGS.get().expect(
        "Mappings are not initialized. \
                Help: call css_mod::init!() once early (eg. in main.rs)",
    );

    // normalize source file path to posix-style (ie. with forward slash separators), because
    // `std::path::Path` parses paths with posix-style logic when been built for wasm target.
    // ie. `file!()` returns path in host-os-style, while `std::path::Path` parses path in
    // target-os-style. eg. when building on windows host `source_file_path` is in windows-style
    // (ie. with backward slash separators), while `std::path::Path` been built for wasm target
    // parses it as posix-style path so doesn't recornize separators. as a result `with_file_name`
    // replaces whole path with `css_module_path`.
    // using `std::path::Path` instead of simple handmade last fragment replacement is important
    // because it allows to merge with relative CSS module path, which is hard to do by hand.
    let source_path_normalized = if mappings.is_windows_host {
        source_path.replace('\\', "/")
    } else {
        source_path.to_owned()
    };

    let module_file_path =
        std::path::Path::new(&source_path_normalized).with_file_name(css_module_path);
    let module_file_path = module_file_path
        .to_str()
        .expect("Failed to read CSS module path");

    mappings
        .map
        .get(module_file_path)
        .unwrap_or_else(|| panic!("CSS module was not found: {:?}", css_module_path))
}
