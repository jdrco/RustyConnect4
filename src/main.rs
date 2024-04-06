mod components;

use crate::components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
