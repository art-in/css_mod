use yew::prelude::*;

#[function_component]
pub fn Footer() -> Html {
    let css = css_mod::get!("footer.css");
    html! {
        <footer class={css["root"]}>
            <p>
                { "Created by " }
                <a href="https://github.com/art-in/" target="_blank">{ "art-in" }</a>
            </p>
            <p>
                { "Inspired by " }
                <a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a>
            </p>
        </footer>
    }
}
