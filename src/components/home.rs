use yew::prelude::*;
use yew::{function_component, html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/connect4")]
    Connect4,
    #[at("/tootandotto")]
    TootAndOtto,
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            {"Welcome to the game portal!"}
        </div>
    }
}
