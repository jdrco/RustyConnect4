use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, html};

const ROWS: usize = 4;
const COLS: usize = 6;

#[function_component]
pub fn TootAndOttoBoard() -> Html {
    let board = use_state(|| vec![vec![(' ', 0); COLS]; ROWS]);
    let player_turn = use_state(|| 1);
    let player_choice = use_state(|| 'T');

    let handle_click = {
        let board = board.clone();
        let player_turn = player_turn.clone();
        let player_choice = player_choice.clone();
        Callback::from(move |x: usize| {
            let mut new_board = (*board).clone();
            if let Some(y) = (0..ROWS).rev().find(|&y| new_board[y][x].0 == ' ') {
                new_board[y][x] = (*player_choice, *player_turn);
                board.set(new_board);
                player_turn.set(if *player_turn == 1 { 2 } else { 1 });
            }
        })
    };

    let handle_option_change = {
        let player_choice = player_choice.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            player_choice.set(input.value().chars().next().unwrap_or('T'));
        })
    };

    html! {
        <div>
            <div class="text-4xl font-bold"><b>{"Enter Your Name"}</b></div>
            <div class="bg-red-500 w-16 h-4 rounded-md"></div>
            <form class="col-md-offset-4 col-md-8">
                <div class="col-md-offset-3 col-md-8">
                    <input id="textbox1" type="text" placeholder="Your Name"/>
                    <button class="bg-violet-500 rounded-md p-2 text-white">
                        {"Save"}
                    </button>
                </div>
            </form>
            <div>
                <input type="radio" id="choose_t" name="player_choice" value="T"
                       checked={*player_choice == 'T'}
                       onchange={handle_option_change.clone()}/>
                <label for="choose_t">{"Choose T"}</label>

                <input type="radio" id="choose_o" name="player_choice" value="O"
                       checked={*player_choice == 'O'}
                       onchange={handle_option_change}/>
                <label for="choose_o">{"Choose O"}</label>
            </div>
            <div class="post">
                <br/>
                <h4>{"Player Turn: "}{if *player_turn == 1 { "Player 1 (Red)" } else { "Player 2 (Yellow)" }}</h4>
                <small>{"Choose 'T' or 'O' to play."}</small>
                <br/>
            </div>
            <div id="gameboard" class="w-[500px] border border-black bg-blue-500">
                { for (0..ROWS).map(|y| html! {
                    <div class="flex justify-center items-center gap-4 my-4">
                        { for (0..COLS).map(|x| html! {
                            <div onclick={handle_click.reform(move |_| x)}
                                 class={
                                    let base_class = "w-12 h-12 rounded-full flex items-center justify-center text-xl text-white";
                                    let color_class = if board[y][x].1 == 1 {
                                        "bg-red-500"
                                    } else if board[y][x].1 == 2 {
                                        "bg-yellow-500"
                                    } else {
                                        "bg-white"
                                    };
                                    format!("{} {}", base_class, color_class)
                                 }>
                                { board[y][x].0.to_string() }
                            </div>
                        })}
                    </div>
                })}
            </div>
        </div>
    }
}
