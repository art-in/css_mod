fn main() {
    css_mod::init!();

    let css = css_mod::get!("styles.css");
    dbg!(css["local-class-name"]);
}
