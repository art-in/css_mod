mod components;
mod state;

use components::App;

fn main() {
    css_mod::init!();
    yew::Renderer::<App>::new().render();
}
