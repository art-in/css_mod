use crate::components::FilterList;
use crate::state::Filter as FilterModel;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub filter: FilterModel,
    pub total: usize,
    pub completed: usize,
    pub on_set_filter: Callback<FilterModel>,
    pub on_clear_completed: Callback<MouseEvent>,
}

#[function_component]
pub fn StatusBar(props: &Props) -> Html {
    let css = css_mod::get!("status_bar.css");
    html! {
        <footer class={classes!(css["root"], props.class.clone())}>
            <span class={css["total-count"]}>
                <span>{ props.total }</span>
                { " item(s) left" }
            </span>
            <FilterList filter={props.filter} on_set_filter={props.on_set_filter.clone()} />
            <button class={css["clear-completed"]} onclick={props.on_clear_completed.clone()}>
                { format!("Clear completed ({})", props.completed) }
            </button>
        </footer>
    }
}
