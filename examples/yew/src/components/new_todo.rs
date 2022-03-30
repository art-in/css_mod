use web_sys::HtmlInputElement;
use yew::events::KeyboardEvent;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub is_all_completed: bool,
    pub is_empty: bool,
    pub on_toggle_all: Callback<MouseEvent>,
    pub on_add: Callback<String>,
}

#[function_component]
pub fn NewTodo(props: &Props) -> Html {
    let css = css_mod::get!("new_todo.css");

    let onkeypress = {
        let onadd = props.on_add.clone();

        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();

                input.set_value("");
                onadd.emit(value);
            }
        }
    };

    html! {
        <header class={css["root"]}>
            <input
                class={css["toggle-all"]}
                id="toggle-all"
                type="checkbox"
                checked={props.is_all_completed}
                onclick={props.on_toggle_all.clone()}
            />
            <label
                for="toggle-all"
                title="Toggle All"
                style={props.is_empty.then(|| "visibility: hidden;")}
            />

            <input
                class={css["input"]}
                placeholder="What needs to be done?"
                {onkeypress}
            />
        </header>
    }
}
