use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub filter: crate::state::Filter,
    pub selected: bool,
    pub on_set_filter: Callback<crate::state::Filter>,
}

#[function_component]
pub fn Filter(props: &Props) -> Html {
    let css = css_mod::get!("filter.css");

    let on_set_filter = {
        let on_set_filter = props.on_set_filter.clone();
        let filter = props.filter;
        move |_| on_set_filter.emit(filter)
    };

    html! {
        <li class={classes!(css["root"], props.selected.then(|| css["selected"]))}>
            <a href={props.filter.as_href()} onclick={on_set_filter}>
                { props.filter }
            </a>
        </li>
    }
}
