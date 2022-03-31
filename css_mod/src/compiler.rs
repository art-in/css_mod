use crate::parsing::ast;
use anyhow::{Context, Result};
use glob::glob;
use quote::quote;
use serde::Deserialize;
use std::{
    collections::HashSet,
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

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
        let path = PathBuf::from(path);

        self.add_module_buf(path)?;

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
    /// Generates CSS bundle file ready to include in HTML page, and name mappings rust code ready
    /// to include into rust project with `css_mod::init!()`.
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

        let workspace_dir = Self::get_workspace_dir()?;
        log::debug!("workspace dir: {:?}", workspace_dir);

        mappings_code_content.push_str(
            &quote! {
                ::css_mod::Mappings::default()
            }
            .to_string(),
        );

        for module in stylesheet.modules.values() {
            for child in &module.children {
                css_bundle_content.push_str(&format!("{}", child));
            }

            // mapping is going to be looked up with module file path as a key (see css_mod::get!).
            // that path key is constructed from file!() macro, which returns path relative to
            // workspace directory. so path constructed here should also be relative to workspace
            debug_assert!(module.file_path.is_absolute());
            let relative_module_path = module
                .file_path
                .strip_prefix(&workspace_dir)?
                .to_str()
                .context("Failed to construct relative module path")?;

            let mut identifiers = Vec::new();

            for (old, new) in &module.names {
                identifiers.push(quote! {(#old, #new)});
            }

            let mapping_code = quote! {
                .add_mapping(
                    #relative_module_path,
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
        Self::write(&css_bundle_path, css_bundle_content)?;

        // output mappings code
        let out_dir = env::var("OUT_DIR").context(
            "OUT_DIR environment variable was not found. \
                Help: CSS modules compilation should run from cargo build script.",
        )?;
        let out_dir = Path::new(&out_dir);

        let mappings_code_file_path = &out_dir.join(crate::MAPPINGS_FILE_NAME!());
        log::debug!("output mappings code: {:?}", mappings_code_file_path);
        Self::write(mappings_code_file_path, mappings_code_content)?;

        Ok(())
    }

    fn write(file_path: &Path, content: String) -> Result<()> {
        let dir_path = file_path
            .parent()
            .context("Failed to get parent directory")?;
        create_dir_all(&dir_path)?;

        let mut file = File::create(&file_path)
            .with_context(|| format!("Failed to create file: {:?}", file_path))?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    /// Gets path to workspace root directory of currently built package, or package root directory
    /// if it is not part of workspace.
    fn get_workspace_dir() -> Result<PathBuf> {
        // this is ugly but the only way to get workspace directory path right now
        // TODO: replace with environment variable when cargo supports it
        // https://github.com/rust-lang/cargo/issues/3946
        #[derive(Deserialize)]
        struct Manifest {
            workspace_root: String,
        }
        let package_dir = env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR")?;
        let output = std::process::Command::new(env!("CARGO"))
            .arg("metadata")
            .arg("--format-version=1")
            .current_dir(&package_dir)
            .output()?;
        let manifest: Manifest = serde_json::from_slice(&output.stdout)?;
        Ok(PathBuf::from(manifest.workspace_root))
    }
}
