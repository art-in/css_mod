fn main() {
    css_mod::Compiler::new()
        .add_modules("src/**/*.css")
        .compile("assets/app.css");
}
