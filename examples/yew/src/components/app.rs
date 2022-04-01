use crate::components::{Footer, Header, NewTodo, StatusBar, TodoList};
use crate::state::{Action, Filter, State};
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

const KEY: &str = "yew.functiontodomvc.self";

#[function_component]
pub fn App() -> Html {
    let css = css_mod::get!("app.css");

    let state = use_reducer(|| State {
        todos: LocalStorage::get(KEY).unwrap_or_else(|_| vec![]),
        filter: Filter::from_location(),
    });

    // Effect
    use_effect_with_deps(
        move |state| {
            LocalStorage::set(KEY, &state.clone().todos).expect("failed to set");
            || ()
        },
        state.clone(),
    );

    // Callbacks
    let on_remove = {
        let state = state.clone();
        Callback::from(move |id: usize| state.dispatch(Action::Remove(id)))
    };

    let on_toggle = {
        let state = state.clone();
        Callback::from(move |id: usize| state.dispatch(Action::Toggle(id)))
    };

    let on_toggle_all = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::ToggleAll))
    };

    let on_clear_completed = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::ClearCompleted))
    };

    let on_add = {
        let state = state.clone();
        Callback::from(move |value: String| {
            state.dispatch(Action::Add(value));
        })
    };

    let on_set_filter = {
        let state = state.clone();
        Callback::from(move |filter: Filter| {
            state.dispatch(Action::SetFilter(filter));
        })
    };

    // Helpers
    let completed = state
        .todos
        .iter()
        .filter(|todo| Filter::Completed.fits(todo))
        .count();

    let is_all_completed = state
        .todos
        .iter()
        .all(|e| state.filter.fits(e) & e.completed);

    let total = state.todos.len();
    let is_empty = state.todos.is_empty();

    html! {
        <>
            <Header />
            <main class={css["main"]}>
                <NewTodo {is_all_completed} {is_empty} {on_toggle_all} {on_add} />
                <TodoList
                    filter={state.filter}
                    todos={state.todos.clone()}
                    {is_empty}
                    {on_toggle}
                    {on_remove}
                />
                <StatusBar
                    filter={state.filter}
                    {total}
                    {completed}
                    {is_empty}
                    {on_set_filter}
                    {on_clear_completed}
                />
            </main>
            <Footer />
        </>
    }
}
