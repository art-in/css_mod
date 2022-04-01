use crate::components::FilterList;
use crate::state::Filter as FilterModel;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub filter: FilterModel,
    pub total: usize,
    pub completed: usize,
    pub is_empty: bool,
    pub on_set_filter: Callback<FilterModel>,
    pub on_clear_completed: Callback<MouseEvent>,
}

#[function_component]
pub fn StatusBar(props: &Props) -> Html {
    let css = css_mod::get!("status_bar.css");
    let shared_css = css_mod::get!("../shared.css");

    html! {
        <footer class={classes!(css["root"], props.is_empty.then(|| shared_css["hidden"]))}>
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
