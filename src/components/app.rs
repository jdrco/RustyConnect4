use crate::components::burgermenu::BurgerMenu;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    // let theme = use_state(|| false);
    // let toggle_theme = {
    //     let theme = theme.clone();
    //     Callback::from(move |_| theme.set(!*theme))
    // };

    // let theme_class = if *theme {
    //     "theme-colorblind"
    // } else {
    //     "theme-regular"
    // };

    html! {
        <BurgerMenu />
    }
}
