use crate::constant::{
    //*columns, *rows, 
    HEADER, RED_BAR};
use gloo_console::log;
use rand::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use std::cmp::{max, min};

const USER: usize = 1;
const COMPUTER: usize = 2;
const EMPTY: usize = 0;
const WINDOW_LENGTH: usize = 4;

#[function_component]
pub fn Connect4Board() -> Html {

    let columns = use_state(|| 7);
    let rows = use_state(|| 6);

    let input_columns = use_state(|| 7); // Holds the input field value for columns
    let input_rows = use_state(|| 6); // Holds the input field value for rows

    let board = use_state(|| vec![vec![0; *columns]; *rows]);
    let winner = use_state(|| None::<usize>);

    let make_computer_move = |board: &mut Vec<Vec<usize>>| {
        // Easy mode can just choose randomly
        // let available_cols: Vec<usize> = (0..*columns)
        //     .filter(|&col| board[0][col] == 0)
        //     .collect();
        // if let Some(&col) = available_cols.choose(&mut rand::thread_rng()) {
        //     if let Some(row) = (0..*rows).rev().find(|&r| board[r][col] == 0) {
        //         log!("Computer picked column:", col);
        //         board[row][col] = 2;
        //     }
        // }
        let (best_col, _) = minimax(board, 4, isize::MIN, isize::MAX, true);
        if let Some(row) = get_next_open_row(board, best_col) {
            log!("Computer picked column:", best_col);
            board[row][best_col] = COMPUTER;
        }
    };

    let handle_click = {
        let board = board.clone();
        let winner = winner.clone();
        let rows = rows.clone(); // Clone for use in this closure
        let columns = columns.clone(); // Clone for use in this closure
        Callback::from(move |x: usize| {
            let mut new_board = (*board).clone();
            if let Some(y) = (0..*rows).rev().find(|&y| new_board[y][x] == 0) {
                log!("User picked column:", x);
                new_board[y][x] = USER;

                // Check if user wins after their move
                if let Some(winner_player) = check_winner(&new_board) {
                    winner.set(Some(winner_player));
                } else {
                    // Make computer move only if there is no winner yet
                    make_computer_move(&mut new_board);
                    if let Some(winner_player) = check_winner(&new_board) {
                        winner.set(Some(winner_player));
                    }
                }
                board.set(new_board);
            }
        })
    };

    let on_rows_change = {
        let input_rows = input_rows.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                input_rows.set(input.value_as_number().max(4.0) as usize);
            }
        })
    };
    
    let on_cols_change = {
        let input_columns = input_columns.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                input_columns.set(input.value_as_number().max(5.0) as usize);
            }
        })
    };

    let on_submit = {
        let board = board.clone();
        let rows = rows.clone();
        let columns = columns.clone();
        let input_rows = input_rows.clone();
        let input_columns = input_columns.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            rows.set(*input_rows);
            columns.set(*input_columns);
            board.set(vec![vec![0; *input_columns]; *input_rows]);
            //let new_board = vec![vec![0; *columns]; *rows];
            //board.set(new_board);
        })
    };

    html! {
        <>
            <form onsubmit={on_submit}>
                <div>
                    <label for="rows_input">{"Rows:"}</label>
                    <input id="rows_input" type="number" min="4" max="10" value={(*input_rows).to_string()} oninput={on_rows_change} />
                </div>
                <div>
                    <label for="cols_input">{"Columns:"}</label>
                    <input id="cols_input" type="number" min="4" max="10" value={(*input_columns).to_string()} oninput={on_cols_change} />
                </div>
                <button type="submit">{"Submit Board Size"}</button>
            </form>
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
                    { for (0..*rows).map(|y| html! {
                        <div class="flex justify-center items-center gap-4 my-4">
                            { for (0..*columns).map(|x| html! {
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
                { if let Some(winner) = *winner {
                    winner_modal(winner)
                } else {
                    html! {}
                }}
            </div>
        </>
    }
}

fn winner_modal(winner: usize) -> Html {
    html! {
        <div class={"modal fixed z-1 left-0 top-0 w-full h-full overflow-auto bg-black bg-opacity-40"}>
            <div class={"modal-content bg-gray-100 mx-auto my-15 p-5 border border-gray-400 w-4/5"}>
                <h3>{ format!("Player {} Wins!", if winner == 1 { "Red" } else { "Yellow" }) }</h3>
                <form>
                    <button class="bg-violet-500 rounded-md p-2 text-white">
                        {"Play Again"}
                    </button>
                </form>
            </div>
        </div>
    }
}

pub fn check_winner(board: &Vec<Vec<usize>>) -> Option<usize> {
    let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
    let rows = board.len(); // This gives you the number of rows
    let columns = board.get(0).map_or(0, |row| row.len()); // This gives you the number of columns in the first row
    for y in 0..rows {
        for x in 0..columns {
            if board[y][x] != 0 {
                let current = board[y][x];
                for (dy, dx) in directions.iter() {
                    let mut count = 1;
                    let mut nx = x as isize + dx;
                    let mut ny = y as isize + dy;
                    while nx >= 0 && nx < columns as isize && ny >= 0 && ny < rows as isize && board[ny as usize][nx as usize] == current {
                        count += 1;
                        if count == 4 {
                            return Some(current);
                        }
                        nx += dx;
                        ny += dy;
                    }
                }
            }
        }
    }
    None
}

fn get_valid_locations(board: &Vec<Vec<usize>>) -> Vec<usize> {
    let columns = board.get(0).map_or(0, |row| row.len()); // This gives you the number of columns in the first row
    (0..columns).filter(|&col| board[0][col] == 0).collect()
}

fn get_next_open_row(board: &Vec<Vec<usize>>, col: usize) -> Option<usize> {
    let rows = board.len(); // This gives you the number of rows
    (0..rows).rev().find(|&row| board[row][col] == 0)
}

fn drop_piece(board: &mut Vec<Vec<usize>>, row: usize, col: usize, piece: usize) {
    let rows = board.len(); // This gives you the number of rows
    if row < rows {
        board[row][col] = piece;
    }
}

fn is_terminal_node(board: &Vec<Vec<usize>>) -> bool {
    check_winner(board).is_some() || get_valid_locations(board).is_empty()
}

fn score_position(board: &Vec<Vec<usize>>, piece: usize) -> isize {
    let rows = board.len(); // This gives you the number of rows
    let columns = board.get(0).map_or(0, |row| row.len()); // This gives you the number of columns in the first row

    let mut score = 0;
    let center_col = columns / 2;
    let center_count = board.iter().map(|row| row[center_col] == piece).count() as isize;
    score += center_count * 100;

    for row in board {
        for col in 0..columns - 3 {
            let window = &row[col..col + 4];
            score += evaluate_window(window, piece);
        }
    }

    for col in 0..columns {
        for row in 0..rows - 3 {
            let window = (0..4).map(|i| board[row + i][col]).collect::<Vec<_>>();
            score += evaluate_window(&window, piece);
        }
    }

    for row in 0..rows - 3 {
        for col in 0..columns - 3 {
            let window = (0..4).map(|i| board[row + i][col + i]).collect::<Vec<_>>();
            score += evaluate_window(&window, piece);
        }
    }

    for row in 3..rows {
        for col in 0..columns - 3 {
            let window = (0..4).map(|i| board[row - i][col + i]).collect::<Vec<_>>();
            score += evaluate_window(&window, piece);
        }
    }

    score
}

pub fn minimax(board: &Vec<Vec<usize>>, depth: usize, mut alpha: isize, mut beta: isize, maximizing_player: bool) -> (usize, isize) {
    if depth == 0 || is_terminal_node(board) {
        return (0, score_position(board, if maximizing_player { 2 } else { 1 }));
    }

    if maximizing_player {
        let mut value = isize::MIN;
        let mut best_column = 0;
        for col in get_valid_locations(board) {
            if let Some(row) = get_next_open_row(board, col) {
                let mut new_board = board.clone();
                drop_piece(&mut new_board, row, col, 2);
                let (_, score) = minimax(&new_board, depth - 1, alpha, beta, false);
                if score > value {
                    value = score;
                    best_column = col;
                }
                alpha = max(alpha, value);
                if alpha >= beta {
                    break;
                }
            }
        }
        return (best_column, value);
    } else {
        let mut value = isize::MAX;
        let mut best_column = 0;
        for col in get_valid_locations(board) {
            if let Some(row) = get_next_open_row(board, col) {
                let mut new_board = board.clone();
                drop_piece(&mut new_board, row, col, 1);
                let (_, score) = minimax(&new_board, depth - 1, alpha, beta, true);
                if score < value {
                    value = score;
                    best_column = col;
                }
                beta = min(beta, value);
                if alpha >= beta {
                    break;
                }
            }
        }
        return (best_column, value);
    }
}

fn evaluate_window(window: &[usize], piece: usize) -> isize {
    let mut score = 0;
    let opp_piece = if piece == USER { COMPUTER } else { USER };
    let count_piece = window.iter().filter(|&&p| p == piece).count();
    let count_empty = window.iter().filter(|&&p| p == 0).count();

    match (count_piece, count_empty) {
        (4, _) => score += 10000,
        (3, 1) => score += 100,
        (2, 2) => score += 10,
        _ => (),
    }

    if window.iter().filter(|&&p| p == opp_piece).count() == 3 && count_empty == 1 {
        score -= 100;
    }

    if window.iter().filter(|&&p| p == opp_piece).count() == 4 {
        score -= 1000;
    }

    score
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
