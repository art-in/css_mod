#[allow(clippy::needless_doctest_main)]
/// Initializes CSS name mappings which is later retreived with `css_mod::get!()`.
///
/// Should be called once in a lifetime of the program, before first call to `css_mod::get!()`.
///
/// # Example
///
/// ```ignore
/// // main.rs
///
/// fn main() {
///     css_mod::init!();
/// }
/// ```
#[macro_export]
macro_rules! init {
    () => {{
        let mappings = include!(concat!(
            env!(
                "OUT_DIR",
                "OUT_DIR environment variable was not found. \
                    Help: setup css_mod::Compiler in build.rs"
            ),
            "/",                   // TODO: is it portable? eg. will it work on windows?
            "css_mod_mappings.rs"  // TODO: move to const static and share, if portable
        ));

        // cannot add friendly error message if mappings file doesn't not exist since include!()
        // does not support custom error messages. mappings file can be absent eg. when user adds
        // build.rs to their project but does not call compiler in it.
        // fix for this is to create custom proc_macro which includes mappings and produces good
        // error message, but i don't want to bother with separate proc_macro crate only because
        // of error messages

        ::css_mod::MAPPINGS.set(mappings).expect(
            "Mappings were already initialized before. \
                Help: call css_mod::init!() once early (eg. in main.rs)",
        );
    }};
}

/// Gets name mapping for concreet CSS module.
///
/// # Example
///
/// ```no_run
/// // my-component.rs
///
/// let css = css_mod::get!("my-component.css");
/// let global_class_name = css["local-class-name"];
/// ```
#[macro_export]
macro_rules! get {
    ($a:expr) => {{
        use ::std::path::PathBuf;
        let css_file = PathBuf::from(file!()).parent().unwrap().join($a);
        ::css_mod::get_mapping(css_file)
    }};
}
