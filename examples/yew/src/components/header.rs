use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
    let css = css_mod::get!("header.css");
    html! {
        <header class={css["root"]}>
            <h1>{"todos"}</h1>
        </header>
    }
}
