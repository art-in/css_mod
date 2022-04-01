use crate::components::Todo;
use crate::state::Filter as FilterModel;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub filter: FilterModel,
    pub todos: Vec<crate::state::Todo>,
    pub is_empty: bool,
    pub on_toggle: Callback<usize>,
    pub on_remove: Callback<usize>,
}

#[function_component]
pub fn TodoList(props: &Props) -> Html {
    let css = css_mod::get!("todo_list.css");
    let shared_css = css_mod::get!("../shared.css");

    html! {
        <section class={classes!(css["root"], props.is_empty.then(|| shared_css["hidden"]))}>
            <ul>
                { for props.todos.iter().filter(|e| props.filter.fits(e)).cloned().map(|todo|
                    html! {
                        <Todo
                            class={css["item"]}
                            {todo}
                            on_toggle={props.on_toggle.clone()}
                            on_remove={props.on_remove.clone()}
                        />
                }) }
            </ul>
        </section>
    }
}
