fn main() {
    css_mod::init!();

    let css = css_mod::get!("styles.css");
    let global_class_name = css["local-class-name"];

    println!("global class name: {}", global_class_name);
}
