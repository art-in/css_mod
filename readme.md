Rust implementation of [CSS Modules]

> A **CSS Module** is a CSS file in which all class names and animation names are scoped locally by default.

# Features

This is currently incomplete implementation of CSS Modules spec, as it only supports vital features.

- [ ] Local scoping for names
    - [x] Classes
    - [x] Animations
    - [ ] Grid lines/areas
    - [ ] `@counter-style`
- [ ] `:local()` / `:global()`
- [ ] `composes`
- [ ] `url()` / `@import`

# Usage

1. Add this crate as a regular and build dependency:

    ```toml
    // Cargo.toml

    [dependencies]
    css_mod = TODO

    [build-dependencies]
    css_mod = TODO
    ```

2. Compile CSS modules in build script:

    ```rust
    // build.rs

    fn main() {
        css_mod::Compiler::new()
            .add_modules("src/**/*.css")
            .compile("assets/app.css");
    }
    ```

3. Initialize somewhere early in program execution:

    ```rust
    // main.rs

    fn main() {
        css_mod::init!();
    }
    ```

4. Finally get global names from CSS modules:

    ```rust
    // my-component.rs

    let css = css_mod::get!("my-component.css");
    let global_class_name = css["local-class-name"];
    ```

[CSS Modules]: https://github.com/css-modules/css-modules
[Parcel CSS]: https://github.com/parcel-bundler/parcel-css
