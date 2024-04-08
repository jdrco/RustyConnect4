use crate::components::burgermenu::BurgerMenu;
use crate::components::connect4board::{Connect4Board, Connect4Rules};
use crate::components::tootandottoboard::{TootAndOttoBoard, TootAndOttoRules};
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let theme = use_state(|| false);

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| theme.set(!*theme))
    };

    let theme_class = if *theme {
        "theme-colorblind"
    } else {
        "theme-regular"
    };

    html! {
        <div class={classes!(theme_class)}>
            <button onclick={toggle_theme} class="px-4 py-2 rounded bg-blue-500 text-white hover:bg-blue-700 focus:outline-none">
                {"Toggle Theme"}
            </button>
            <BurgerMenu />
            <Connect4Rules />
            <Connect4Board />
            <TootAndOttoRules />
            <TootAndOttoBoard />
        </div>
    }
}
