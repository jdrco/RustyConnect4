use crate::components::burgermenu::BurgerMenu;
use crate::components::connect4board::{Connect4Board, Connect4Rules};
use crate::components::tootandottoboard::{TootAndOttoBoard, TootAndOttoRules};
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <BurgerMenu />
            <Connect4Rules />
            <Connect4Board />
            <TootAndOttoRules />
            <TootAndOttoBoard />
        </>
    }
}
