use gloo::history::BrowserHistory;
use gloo::history::History;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use strum_macros::Display;
use strum_macros::EnumIter;
use yew::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub todos: Vec<Todo>,
    pub filter: Filter,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn fits(&self, todo: &Todo) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !todo.completed,
            Filter::Completed => todo.completed,
        }
    }

    pub fn as_href(&self) -> &'static str {
        match self {
            Filter::All => "#/",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        }
    }

    pub fn from_location() -> Self {
        let location = BrowserHistory::new().location();
        match location.hash() {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
    }
}

pub enum Action {
    Add(String),
    Remove(usize),
    SetFilter(Filter),
    Toggle(usize),
    ToggleAll,
    ClearCompleted,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Add(description) => {
                let mut todos = self.todos.clone();
                todos.push(Todo {
                    id: todos.last().map(|todo| todo.id + 1).unwrap_or(1),
                    description,
                    completed: false,
                });
                State {
                    todos,
                    filter: self.filter,
                }
                .into()
            }
            Action::Remove(id) => {
                let mut todos = self.todos.clone();
                todos.retain(|todo| todo.id != id);
                State {
                    todos,
                    filter: self.filter,
                }
                .into()
            }
            Action::SetFilter(filter) => State {
                filter,
                todos: self.todos.clone(),
            }
            .into(),
            Action::Toggle(id) => {
                let mut todos = self.todos.clone();
                let todo = todos.iter_mut().find(|todo| todo.id == id);
                if let Some(todo) = todo {
                    todo.completed = !todo.completed;
                }
                State {
                    todos,
                    filter: self.filter,
                }
                .into()
            }
            Action::ToggleAll => {
                let mut todos = self.todos.clone();
                let is_all_completed = self.todos.iter().all(|e| self.filter.fits(e) & e.completed);
                for todo in &mut todos {
                    if self.filter.fits(todo) {
                        todo.completed = !is_all_completed;
                    }
                }
                State {
                    todos,
                    filter: self.filter,
                }
                .into()
            }
            Action::ClearCompleted => {
                let mut todos = self.todos.clone();
                todos.retain(|e| Filter::Active.fits(e));
                State {
                    todos,
                    filter: self.filter,
                }
                .into()
            }
        }
    }
}
