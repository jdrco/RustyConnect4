use crate::components::connect4board::Connect4board;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <Connect4board />
        </>
    }
}
