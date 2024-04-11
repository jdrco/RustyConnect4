use crate::constant::EMPTY;
use yew::prelude::*;
use yew::{function_component, html};

#[derive(Properties, PartialEq)]
pub struct WinnerModalProps {
    pub winner: usize,
}

#[function_component]
pub fn WinnerModal(props: &WinnerModalProps) -> Html {
    html! {
        <div class={"modal fixed z-1 left-0 top-0 w-full h-full overflow-auto bg-black bg-opacity-40"}>
            <div class={"modal-content bg-gray-100 mx-auto my-15 p-5 border border-gray-400 w-4/5"}>
                {
                    if props.winner == EMPTY {
                        html! {
                            <h3>{"It's a Draw!"}</h3>
                        }
                    } else {
                        html! {
                            <h3>{ format!("Player {} Wins!", props.winner)}</h3>
                        }
                    }
                }
                <form>
                    <button class="bg-violet-500 rounded-md p-2 text-white">
                        {"Play Again"}
                    </button>
                </form>
            </div>
        </div>
    }
}
