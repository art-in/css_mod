use crate::components::Todo;
use crate::state::Filter as FilterModel;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub filter: FilterModel,
    pub todos: Vec<crate::state::Todo>,
    pub on_toggle: Callback<usize>,
    pub on_remove: Callback<usize>,
}

#[function_component]
pub fn TodoList(props: &Props) -> Html {
    let css = css_mod::get!("todo_list.css");

    html! {
        <section class={classes!(css["root"], props.class.clone())}>
            <ul>
                { for props.todos.iter().filter(|e| props.filter.fits(e)).cloned().map(|todo|
                    html! {
                        <Todo
                            {todo}
                            on_toggle={props.on_toggle.clone()}
                            on_remove={props.on_remove.clone()}
                        />
                }) }
            </ul>
        </section>
    }
}
