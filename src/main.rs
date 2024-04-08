mod components;
mod constant;

// use crate::components::app::App;

// use crate::components::burgermenu::BurgerMenu;
use crate::components::connect4board::{Connect4Board, Connect4Rules};
use crate::components::tootandottoboard::{TootAndOttoBoard, TootAndOttoRules};
use yew::html::Scope;
use yew::prelude::*;

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

pub enum Msg {
    ToggleNavbar,
}

pub struct App {
    navbar_active: bool,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        }
    }
}
impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Connect4Board}>
                            { "Connect 4 Board" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
        }
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

fn main() {
    yew::Renderer::<App>::new().render();
}

// fn main() {
//     yew::Renderer::<App>::new().render();
// }
