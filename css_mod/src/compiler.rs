use crate::parsing::ast;
use crate::utils::{get_workspace_dir, write_file};
use anyhow::{Context, Result};
use glob::glob;
use quote::quote;
use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};

#[allow(clippy::needless_doctest_main)]
/// CSS Modules compiler.
///
/// Intended to be used inside build script.
///
/// # Example:
///
/// ```no_run
/// // build.rs
///
/// fn main() {
///     css_mod::Compiler::new()
///         .add_modules("src/**/*.css").unwrap()
///         .compile("assets/app.css").unwrap();
/// }
/// ```
#[derive(Debug, Default)]
pub struct Compiler {
    input_modules: HashSet<PathBuf>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler::default()
    }

    /// Adds CSS module to compile.
    ///
    /// Arguments:
    ///
    /// * `path`: File path, which may be absolute or relative to package root directory.
    pub fn add_module(&mut self, path: &str) -> Result<&mut Self> {
        self.add_module_buf(PathBuf::from(path))?;
        Ok(self)
    }

    /// Adds CSS modules to compile.
    ///
    /// Arguments:
    ///
    /// * `pattern`: Glob pattern, which may be absolute or relative to package root directory.
    pub fn add_modules(&mut self, pattern: &str) -> Result<&mut Self> {
        for entry in glob(pattern).context("Failed to read glob pattern")? {
            self.add_module_buf(entry?)?;
        }
        Ok(self)
    }

    fn add_module_buf(&mut self, mut path: PathBuf) -> Result<()> {
        if path.is_relative() {
            let manifest_dir = env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR")?;
            path = PathBuf::from(manifest_dir).join(path);
        }

        log::debug!("add css module: {:?}", path);
        self.input_modules.insert(path);

        Ok(())
    }

    /// Parses and transforms input CSS modules.
    ///
    /// Generates CSS bundle file ready to be included on a HTML page, and rust code file with name
    /// mappings ready to be included into rust project with [`css_mod::init!`](crate::init).
    ///
    /// Arguments:
    ///
    /// * `css_bundle_path`: File path for output CSS bundle, which may be absolute or relative to
    ///                      package root directory.
    pub fn compile(&self, css_bundle_path: &str) -> Result<()> {
        // parse and transform input CSS files
        let mut stylesheet = ast::Stylesheet::default();

        for module_path in &self.input_modules {
            stylesheet.add_module(module_path).with_context(|| {
                format!(
                    "Failed to parse and transform CSS module: {:?}",
                    module_path
                )
            })?;
        }

        // generate contents for css bundle and mappings code files
        let mut css_bundle_content = String::new();
        let mut mappings_code_content = String::new();

        let workspace_dir = get_workspace_dir()?;
        log::debug!("workspace dir: {:?}", workspace_dir);

        let is_windows_host = cfg!(windows);
        log::debug!("is windows host: {}", is_windows_host);

        mappings_code_content.push_str(
            // save host os info into mappings since it's impossible to get it outside of build script
            &quote! {
                ::css_mod::Mappings::default().is_windows_host(#is_windows_host)
            }
            .to_string(),
        );

        for module in stylesheet.modules.values() {
            for child in &module.children {
                css_bundle_content.push_str(&format!("{}", child));
            }

            // css_mod::get!() will look up name mapping with module file path as a key. that path
            // is constructed from file!() macro, which returns path relative to workspace
            // directory. so make sure constructed module path key is relative to workspace
            debug_assert!(module.file_path.is_absolute());
            let mut module_file_path = module
                .file_path
                .strip_prefix(&workspace_dir)?
                .to_str()
                .context("Failed to construct relative module path")?
                .to_owned();

            // css_mod::get!() will receive posix-style path (ie. with forward slash separators).
            // so make sure constructed module path key is normalized to posix-style
            if is_windows_host {
                module_file_path = module_file_path.replace('\\', "/");
            }

            let mut identifiers = Vec::new();

            for (old, new) in &module.names {
                identifiers.push(quote! {(#old, #new)});
            }

            let mapping_code = quote! {
                .add_mapping(
                    #module_file_path,
                    [#(#identifiers),*],
                )
            };

            mappings_code_content.push_str(&mapping_code.to_string());
        }

        // output css bundle
        let mut css_bundle_path = PathBuf::from(css_bundle_path);
        if css_bundle_path.is_relative() {
            let package_dir = env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR")?;
            let package_dir = Path::new(&package_dir);

            css_bundle_path = package_dir.join(&css_bundle_path);
        }

        log::debug!("output css bundle: {:?}", css_bundle_path);
        write_file(&css_bundle_path, css_bundle_content)?;

        // output mappings code
        let out_dir = env::var("OUT_DIR").context(
            "OUT_DIR environment variable was not found. \
                Help: CSS modules compilation should run from cargo build script.",
        )?;
        let out_dir = Path::new(&out_dir);

        let mappings_code_file_path = &out_dir.join(crate::MAPPINGS_FILE_NAME!());
        log::debug!("output mappings code: {:?}", mappings_code_file_path);
        write_file(mappings_code_file_path, mappings_code_content)?;

        Ok(())
    }
}
