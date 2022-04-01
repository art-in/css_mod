use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn write_file(file_path: &Path, content: String) -> Result<()> {
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
pub fn get_workspace_dir() -> Result<PathBuf> {
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

// Resolves CSS module file path.
//
// * `source_path`: Source code file path from which CSS module is requested.
//      In host-os-style (ie. on windows - with backward, otherwise - forward slash separators).
//      Expected to be result of `file!()` macro.
// * `css_module_path`: CSS module file path relative to source file.
//      In posix-style (ie. with forward slash separators).
pub fn resolve_module_file_path(
    source_path: &str,
    css_module_path: &str,
    is_windows_host: bool,
) -> String {
    // normalize source path separators to posix-style, since `file!()` returns host-os-style paths.
    // not using cfg!(windows) here because it corresponds to target (which is 'wasm' when
    // been built for browser), not host os on which building is happening
    let source_path_normalized = if is_windows_host {
        source_path.replace('\\', "/")
    } else {
        source_path.to_owned()
    };

    join_paths(&source_path_normalized, css_module_path)
}

/// Joins file paths
///
/// Intention is to perform path joining with basic two-dot normalization as fast as possible.
/// Two-dot normalization will only happen for two-dots at the beginning of rhs path.
/// Both paths expected to be in posix-style (ie. with forward slash separators).
fn join_paths(lhs: &str, rhs: &str) -> String {
    let mut lhs = lhs.trim_end_matches(|c| c != '/');
    let mut rhs = rhs;

    while rhs.starts_with("../") {
        lhs = lhs.trim_end_matches(|c| c != '/');
        lhs = lhs.strip_suffix('/').unwrap_or("");
        lhs = lhs.trim_end_matches(|c| c != '/');
        rhs = rhs.strip_prefix("../").unwrap_or("");
    }

    lhs.to_owned() + rhs
}

#[cfg(test)]
mod join_paths {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(join_paths("a/b/c", "d"), "a/b/d");
        assert_eq!(join_paths("a/b/", "d"), "a/b/d");
        assert_eq!(join_paths("a/b/", "c/d"), "a/b/c/d");
    }

    #[test]
    fn two_dot_normalization() {
        assert_eq!(join_paths("a/b/", "../d"), "a/d");
        assert_eq!(join_paths("a/b/c", "../d"), "a/d");
        assert_eq!(join_paths("a/b/c", "../../d"), "d");
        assert_eq!(join_paths("a/b/c", "../../../d"), "d");
    }

    #[test]
    fn exceptions() {
        // doesn't normalize one dot
        assert_eq!(join_paths("a/./b/c", "./d"), "a/./b/./d");

        // doesn't normalize two dots not at the beginning of rhs
        assert_eq!(join_paths("a/../b/", "c/../d"), "a/../b/c/../d");

        // doesn't work with back slash separators
        assert_eq!(join_paths("a\\b\\c", "d"), "d");
        assert_eq!(join_paths("a/b/c", "..\\d"), "a/b/..\\d");
    }
}
