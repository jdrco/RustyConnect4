mod components;
mod constant;

use crate::components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
