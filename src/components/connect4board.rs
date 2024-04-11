use crate::constant::{DEFAULT_C4_COLS, DEFAULT_C4_ROWS, HEADER, RED_BAR};
use gloo_console::log;
use rand::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn Connect4Board() -> Html {
    let board = use_state(|| vec![vec![0; DEFAULT_C4_COLS]; DEFAULT_C4_ROWS]);

    let make_computer_move = |board: &mut Vec<Vec<usize>>| {
        let available_cols: Vec<usize> = (0..DEFAULT_C4_COLS)
            .filter(|&col| board[0][col] == 0)
            .collect();

        if let Some(&col) = available_cols.choose(&mut rand::thread_rng()) {
            if let Some(row) = (0..DEFAULT_C4_ROWS).rev().find(|&r| board[r][col] == 0) {
                log!("Computer picked column:", col);
                board[row][col] = 2;
            }
        }
    };

    let handle_click = {
        let board = board.clone();
        Callback::from(move |x: usize| {
            let mut new_board = (*board).clone();
            if let Some(y) = (0..DEFAULT_C4_ROWS).rev().find(|&y| new_board[y][x] == 0) {
                log!("User picked column:", x);
                new_board[y][x] = 1;
                make_computer_move(&mut new_board);
                board.set(new_board);
            }
        })
    };

    html! {
        <>
            <div class={HEADER}><b>{"Enter Your Name"}</b></div>
            <div class={RED_BAR}></div>
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
            <div id="gameboard" class="w-[500px] border border-black bg-boardPrimaryBg">
                { for (0..DEFAULT_C4_ROWS).map(|y| html! {
                    <div class="flex justify-center items-center gap-4 my-4">
                        { for (0..DEFAULT_C4_COLS).map(|x| html! {
                            <div onclick={handle_click.reform(move |_| x)}
                                 class={
                                    let base_class = "w-12 h-12 rounded-full flex items-center justify-center";
                                    match board[y][x] {
                                        1 => format!("{} {}", base_class, "bg-chipPrimaryBg"),
                                        2 => format!("{} {}", base_class, "bg-chipSecondaryBg"),
                                        _ => format!("{} {}", base_class, "bg-white"),
                                    }
                                 }>
                            </div>
                        })}
                    </div>
                })}
            </div>
            </div>
        </>
    }
}

#[function_component]
pub fn Connect4Rules() -> Html {
    html! {
        <div id="main">
            <div class="container mx-auto mt-12" id="services">
                <h5 class={HEADER}><b>{"How to Play Connect 4"}</b></h5>
                <div class={RED_BAR}></div>
                <p>{"Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}</p>
                <br/>
                <div><h5>{"To play Connect 4 follow the following steps:"}</h5></div>
                <ul>
                    <li>{"A new game describes discs of which color belongs to which player"}</li>
                    <li>{"Click on the desired column on the game board to place your disc"}</li>
                    <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>
                </ul>
                <br/>
                <p>{"For More information on Connect 4 click "}<a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a></p>
            </div>
        </div>
    }
}
