use yew::prelude::*;
use yew::{function_component, html};

const ROWS: usize = 6;
const COLS: usize = 7;

#[function_component]
pub fn Connect4board() -> Html {
    let board = use_state(|| vec![vec![0; COLS]; ROWS]);
    let player_turn = use_state(|| true);

    let handle_click = {
        let board = board.clone();
        let player_turn = player_turn.clone();
        Callback::from(move |x: usize| {
            let mut new_board = (*board).clone();
            if let Some(y) = (0..ROWS).rev().find(|&y| new_board[y][x] == 0) {
                new_board[y][x] = if *player_turn { 1 } else { 2 };
                board.set(new_board);
                player_turn.set(!*player_turn);
            }
        })
    };

    html! {
        <div id="">

            <div class="text-4xl font-bold"><b>{"Enter Your Name"}</b></div>
            <div class="bg-red-500 w-16 h-4 rounded-md"></div>
            <div class="col-md-offset-4 col-md-8">
            <form>
                <div class="col-md-offset-3 col-md-8">
                <input id="textbox1" type="text" placeholder="Your Name"/>
                <button class="bg-violet-500 rounded-md p-2 text-white">
                    {"Save"}
                </button>

                </div>
            </form>
            <div class="post">
                <br/>
                <h4>{"New Game: "}</h4>
                <small>{"Disc Colors: Red (You) vs Yellow (Computer)"}</small>
                <br/>
            </div>
            <div id="gameboard" class="w-[500px] border border-black bg-blue-500">
                { for (0..ROWS).map(|y| html! {
                    <div class="flex justify-center items-center gap-4 my-4">
                        { for (0..COLS).map(|x| html! {
                            <div onclick={handle_click.reform(move |_| x)}
                                 class={
                                    let base_class = "w-12 h-12 rounded-full flex items-center justify-center";
                                    match board[y][x] {
                                        1 => format!("{} {}", base_class, "bg-red-500"),
                                        2 => format!("{} {}", base_class, "bg-yellow-500"),
                                        _ => format!("{} {}", base_class, "bg-white"),
                                    }
                                 }>
                            </div>
                        })}
                    </div>
                })}
            </div>
            </div>
        </div>
    }
}
