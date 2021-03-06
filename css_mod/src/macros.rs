// TODO: replace with simple contant when rust supports using them as arguments for std macros
// https://github.com/rust-lang/rust/issues/53749
#[macro_export]
#[doc(hidden)]
macro_rules! MAPPINGS_FILE_NAME {
    () => {
        "css_mod_mappings.rs"
    };
}

#[allow(clippy::needless_doctest_main)]
/// Initializes CSS name mappings which later can be retreived with [`css_mod::get!`](crate::get).
///
/// Should be called once in a lifetime of the program before first call to `css_mod::get!()`.
/// Expects mappings where already generated by [`css_mod::Compiler`](crate::Compiler) in build script.
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
            "/",
            ::css_mod::MAPPINGS_FILE_NAME!()
        ));

        // cannot add friendly error message if mappings file doesn't not exist since include!()
        // does not support custom error messages. mappings file can be absent eg. when user adds
        // build.rs to their project but does not call compiler in it.
        // fix for this is to create custom proc_macro which includes mappings and produces good
        // error message, but i don't want to bother with separate proc_macro crate only because
        // of error messages.

        ::css_mod::MAPPINGS.set(mappings).expect(
            "Mappings were already initialized before. \
                Help: call css_mod::init!() once early (eg. in main.rs)",
        );
    }};
}

/// Gets name mapping for CSS module.
///
/// Expects mappings were initialized earlier with [`css_mod::init!`](crate::init).
///
/// # Arguments
///
/// * `file_path`: relative path to CSS module in posix-style (ie. with forward slash separators).
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
    ($file_path:expr) => {{
        ::css_mod::get_mapping(file!(), $file_path)
    }};
}
