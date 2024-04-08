use crate::components::burgermenu::BurgerMenu;
use crate::components::connect4board::{Connect4Board, Connect4Rules};
use crate::components::home::Home;
use crate::components::tootandottoboard::{TootAndOttoBoard, TootAndOttoRules};
use yew::prelude::*;
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
        <BrowserRouter>
            <div class={classes!(theme_class)}>
                <button onclick={toggle_theme} class="px-4 py-2 rounded bg-blue-500 text-white hover:bg-blue-700 focus:outline-none">
                    {"Toggle Theme"}
                </button>
                <nav>
                    <Link<Route> to={Route::Connect4}>{"Play Connect4"}</Link<Route>>
                    <Link<Route> to={Route::TootAndOtto}>{"Play Toot and Otto"}</Link<Route>>
                </nav>
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Connect4 => html! { <><Connect4Board/><Connect4Rules/></>},
        Route::TootAndOtto => html! { <><TootAndOttoBoard /><TootAndOttoRules/></> },
    }
}
