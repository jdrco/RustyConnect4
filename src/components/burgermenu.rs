use yew::prelude::*;
use yew::{function_component, html, Callback};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/connect4")]
    Connect4Board,
    #[at("/connect4rules")]
    Connect4Rules,
    #[at("/tootandottoboard")]
    TootAndOttoBoard,
    #[at("/tootandottorules")]
    TootAndOttoRules,
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub is_hidden: bool,
}

#[function_component(BurgerMenu)]
pub fn burger_menu(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick}>{"Home"}</button>
        }
    };

    let go_to_connect_4_board_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Connect4Board));
        html! {
            <button {onclick}>{"Connect 4"}</button>
        }
    };

    let go_to_connect_4_rules_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Connect4Rules));
        html! {
            <button {onclick}>{"Connect 4 Rules"}</button>
        }
    };

    let go_to_toot_and_otto_board_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::TootAndOttoBoard));
        html! {
            <button {onclick}>{"TOOT and OTTO"}</button>
        }
    };

    let go_to_toot_and_otto_board_rules = {
        let onclick = Callback::from(move |_| navigator.push(&Route::TootAndOttoRules));
        html! {
            <button {onclick}>{"TOOT and OTTO Rules"}</button>
        }
    };

    let is_hidden = use_state(|| props.is_hidden);

    let toggle_visibility = {
        let is_hidden = is_hidden.clone();
        Callback::from(move |_| {
            is_hidden.set(!*is_hidden);
        })
    };

    html! {
        <div class="relative">
            <button onclick={toggle_visibility} class={format!("block {} hover:text-black focus:outline-none", if *is_hidden {"text-gray-500"} else {"text-black"})}>
                <svg class="h-6 w-6 fill-current" viewBox="0 0 24 24">
                    <path fill-rule="evenodd" clip-rule="evenodd" d="M1 4.5C1 3.67157 1.67157 3 2.5 3H21.5C22.3284 3 23 3.67157 23 4.5C23 5.32843 22.3284 6 21.5 6H2.5C1.67157 6 1 5.32843 1 4.5ZM1 11.5C1 10.6716 1.67157 10 2.5 10H21.5C22.3284 10 23 10.6716 23 11.5C23 12.3284 22.3284 13 21.5 13H2.5C1.67157 13 1 12.3284 1 11.5ZM1 18.5C1 17.6716 1.67157 17 2.5 17H21.5C22.3284 17 23 17.6716 23 18.5C23 19.3284 22.3284 20 21.5 20H2.5C1.67157 20 1 19.3284 1 18.5Z"/>
                </svg>
            </button>
            <div class="md:flex md:items-center md:justify-between ">
                <div class={format!("flex flex-col {}", if *is_hidden {"hidden"} else {""})}>
                    {go_home_button}
                    {go_to_connect_4_board_button}
                    {go_to_connect_4_rules_button}
                    {go_to_toot_and_otto_board_button}
                    {go_to_toot_and_otto_board_rules}
                </div>
            </div>
        </div>
    }
}
