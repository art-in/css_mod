use crate::components::Filter;
use crate::state::Filter as FilterModel;
use strum::IntoEnumIterator;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub filter: FilterModel,
    pub on_set_filter: Callback<FilterModel>,
}

#[function_component]
pub fn FilterList(props: &Props) -> Html {
    let css = css_mod::get!("filter_list.css");
    html! {
        <ul class={css["root"]}>
            { for FilterModel::iter().map(|filter| {
                html! {
                    <Filter {filter}
                        selected={props.filter == filter}
                        on_set_filter={props.on_set_filter.clone()}
                    />
                }
            }) }
        </ul>
    }
}
