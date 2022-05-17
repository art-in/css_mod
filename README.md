[![ci](https://github.com/art-in/css_mod/actions/workflows/ci.yml/badge.svg)](https://github.com/art-in/css_mod/actions/workflows/ci.yml)
[![crate](https://img.shields.io/crates/v/css_mod.svg)](https://crates.io/crates/css_mod)
![rustc version](https://img.shields.io/badge/rustc-stable-lightgrey.svg)

[CSS Modules] implementation for Rust web applications

> A **CSS Module** is a CSS file in which all class names and animation names are scoped locally by default.

# Features

This is currently incomplete implementation of [CSS Modules] spec, as it only supports the vital features.

-   [ ] Local scoping for names
    -   [x] Classes
    -   [x] Animations
    -   [ ] Grid lines/areas
    -   [ ] `@counter-style`
-   [ ] `:local()` / `:global()`
-   [ ] `composes`
-   [ ] `url()` / `@import`

# Usage

1. Add this crate as a regular and build dependency:

    ```toml
    # Cargo.toml

    [dependencies]
    css_mod = "0.1.0"

    [build-dependencies]
    css_mod = "0.1.0"
    ```

2. Create build script and call compiler:

    ```rust
    // build.rs

    fn main() {
        css_mod::Compiler::new()
            .add_modules("src/**/*.css").unwrap()
            .compile("assets/app.css").unwrap();
    }
    ```

3. Call init somewhere early in program execution:

    ```rust
    // src/main.rs

    fn main() {
        css_mod::init!();
    }
    ```

4. Finally get name mapping for CSS module:

    ```rust
    // src/my-component.rs

    let css = css_mod::get!("my-component.css");
    let global_class_name = css["local-class-name"]; // my-component__local-class-name__0
    ```

# Examples

Look in the [examples](./examples/) directory.

[css modules]: https://github.com/css-modules/css-modules
