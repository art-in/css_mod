fn main() {
    css_mod::Compiler::new()
        .add_modules("src/**/*.css")
        .unwrap()
        .compile("assets/app.css")
        .unwrap();
}
