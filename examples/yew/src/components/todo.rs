use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub todo: crate::state::Todo,
    pub on_toggle: Callback<usize>,
    pub on_remove: Callback<usize>,
}

#[function_component]
pub fn Todo(props: &Props) -> Html {
    let css = css_mod::get!("todo.css");

    let on_toggle = {
        let on_toggle = props.on_toggle.clone();
        let id = props.todo.id;
        move |_| on_toggle.emit(id)
    };

    let on_remove = {
        let on_remove = props.on_remove.clone();
        let id = props.todo.id;
        move |_| on_remove.emit(id)
    };

    let mut class = Classes::from(css["root"]);
    if props.todo.completed {
        class.push(css["completed"]);
    }

    html! {
        <li {class}>
            <input
                class={css["toggle"]}
                type="checkbox"
                checked={props.todo.completed}
                onclick={on_toggle}
            />
            <span class={css["description"]}>
                { &props.todo.description }
            </span>
            <button class={css["remove"]} onclick={on_remove} />
        </li>
    }
}
