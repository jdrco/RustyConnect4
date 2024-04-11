use crate::components::connect4board::Connect4Board;
use crate::components::connect4rules::Connect4Rules;
use crate::components::tootandottoboard::{TootAndOttoBoard, TootAndOttoRules};
use yew::prelude::*;
use yew::{function_component, html, Callback};
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub is_hidden: bool,
}

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

#[function_component(BurgerMenu)]
pub fn burger_menu(props: &Props) -> Html {
    let theme = use_state(|| Theme::Regular);
    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            theme.set(match *theme {
                Theme::Regular => Theme::Colorblind,
                Theme::Colorblind => Theme::Protanopia,
                Theme::Protanopia => Theme::Deuteranopia,
                Theme::Deuteranopia => Theme::Tritanopia,
                Theme::Tritanopia => Theme::Regular,
            })
        })
    };

    let theme_class = match *theme {
        Theme::Regular => "theme-regular",
        Theme::Colorblind => "theme-colorblind",
        Theme::Protanopia => "theme-protanopia",
        Theme::Deuteranopia => "theme-deuteranopia",
        Theme::Tritanopia => "theme-tritanopia",        
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
            <button onclick={toggle_theme} class="px-4 py-2 rounded bg-blue-500 text-white hover:bg-blue-700 focus:outline-none">
                {"Toggle Theme"}
            </button>
            <button onclick={toggle_visibility} class={format!("block {} hover:text-black focus:outline-none", if *is_hidden {"text-gray-500"} else {"text-black"})}>
                <svg class="h-6 w-6 fill-current" viewBox="0 0 24 24">
                    <path fill-rule="evenodd" clip-rule="evenodd" d="M1 4.5C1 3.67157 1.67157 3 2.5 3H21.5C22.3284 3 23 3.67157 23 4.5C23 5.32843 22.3284 6 21.5 6H2.5C1.67157 6 1 5.32843 1 4.5ZM1 11.5C1 10.6716 1.67157 10 2.5 10H21.5C22.3284 10 23 10.6716 23 11.5C23 12.3284 22.3284 13 21.5 13H2.5C1.67157 13 1 12.3284 1 11.5ZM1 18.5C1 17.6716 1.67157 17 2.5 17H21.5C22.3284 17 23 17.6716 23 18.5C23 19.3284 22.3284 20 21.5 20H2.5C1.67157 20 1 19.3284 1 18.5Z"/>
                </svg>
            </button>
            <div class="md:flex md:items-center md:justify-between ">
                    <BrowserRouter>
                        <div class={classes!(theme_class)}>
                            <div class={format!("flex flex-col z-50 {}", if *is_hidden {"hidden"} else {""})}>
                                <nav>
                                    <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                                            { "Home" }
                                    </Link<Route>>
                                    <Link<Route> classes={classes!("navbar-item")} to={Route::Connect4Board}>
                                        { "Connect 4 Board" }
                                    </Link<Route>>
                                    <Link<Route> classes={classes!("navbar-item")} to={Route::Connect4Rules}>
                                        { "Connect 4 Rules" }
                                    </Link<Route>>
                                    <Link<Route> classes={classes!("navbar-item")} to={Route::TootAndOttoBoard}>
                                        { "TOOT and OTTO Board" }
                                    </Link<Route>>
                                    <Link<Route> classes={classes!("navbar-item")} to={Route::TootAndOttoRules}>
                                        { "TOOT and OTTO Rules" }
                                    </Link<Route>>
                                </nav>
                            </div>
                            <Switch<Route> render={switch} />
                        </div>
                    </BrowserRouter>
            </div>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Connect4Rules /> }
        }
        Route::Connect4Board => {
            html! { <Connect4Board /> }
        }
        Route::Connect4Rules => {
            html! { <Connect4Rules /> }
        }
        Route::TootAndOttoBoard => {
            html! { <TootAndOttoBoard /> }
        }
        Route::TootAndOttoRules => {
            html! { <TootAndOttoRules /> }
        }
        Route::NotFound => {
            html! {}
        }
    }
}

enum Theme {
    Regular,
    Colorblind,
    Protanopia,
    Deuteranopia,
    Tritanopia,
}