use crate::components::winner_modal::WinnerModal;
use crate::constant::{DEFAULT_C4_COLS, DEFAULT_C4_ROWS};
use gloo_console::log;
use rand::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use std::cmp::{max, min};

const USER: usize = 1;
const COMPUTER: usize = 2;

#[function_component]
pub fn Connect4Board() -> Html {
    let board = use_state(|| vec![vec![0; DEFAULT_C4_COLS]; DEFAULT_C4_ROWS]);
    let winner = use_state(|| None::<usize>);
    let difficulty = use_state(|| "Easy".to_string());

    let make_computer_move = |board: &mut Vec<Vec<usize>>, difficulty: &String| {
        if *difficulty == "Easy" {
            let available_cols: Vec<usize> = (0..DEFAULT_C4_COLS)
                .filter(|&col| board[0][col] == 0)
                .collect();
            if let Some(&col) = available_cols.choose(&mut rand::thread_rng()) {
                if let Some(row) = (0..DEFAULT_C4_ROWS).rev().find(|&r| board[r][col] == 0) {
                    log!("Computer picked column:", col);
                    board[row][col] = 2;
                }
            }
        } else {
            let (best_col, _) = minimax(board, 5, isize::MIN, isize::MAX, true);
            if let Some(row) = get_next_open_row(board, best_col) {
                log!("Computer picked column:", best_col);
                board[row][best_col] = COMPUTER;
            }
        }
    };

    let handle_user_move = {
        let board = board.clone();
        let winner = winner.clone();
        let difficulty = difficulty.clone();
        Callback::from(move |x: usize| {
            let mut new_board = (*board).clone();
            if let Some(y) = (0..DEFAULT_C4_ROWS).rev().find(|&y| new_board[y][x] == 0) {
                log!("User picked column:", x);
                new_board[y][x] = USER;

                // Check if user wins after their move
                if let Some(winner_player) = check_winner(&new_board) {
                    winner.set(Some(winner_player));
                } else {
                    // Make computer move only if there is no winner yet
                    make_computer_move(&mut new_board, &difficulty.clone());
                    if let Some(winner_player) = check_winner(&new_board) {
                        winner.set(Some(winner_player));
                    }
                }
                board.set(new_board);
            }
        })
    };

    let handle_difficulty_change = {
        let difficulty = difficulty.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            difficulty.set(input.value());
        })
    };

    html! {
        <>
            <div class="">
                <div class="post">
                    <div>{"Game"}</div>
                    <dive>{"Disc Colors: Red (You) vs Yellow (Computer)"}</dive>
                </div>
                <div>
                    <input type="radio" name="difficulty_easy" value="Easy"
                        checked={*difficulty == "Easy"}
                        onchange={handle_difficulty_change.clone()}/>
                    <label for="difficulty_easy">{"Easy mode"}</label>

                    <input type="radio" name="difficulty_hard" value="Hard"
                        checked={*difficulty == "Hard"}
                        onchange={handle_difficulty_change}/>
                    <label for="difficulty_hard">{"Hard mode (Play against minimax AI)"}</label>
                </div>
                <div id="gameboard" class="w-[500px] border border-black bg-boardPrimaryBg">
                    { for (0..DEFAULT_C4_ROWS).map(|y| html! {
                        <div class="flex justify-center items-center gap-4 my-4">
                            { for (0..DEFAULT_C4_COLS).map(|x| html! {
                                <div onclick={handle_user_move.reform(move |_| x)}
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
                    html! {<WinnerModal winner={winner} />}
                } else {
                    html! {}
                }}
            </div>
        </>
    }
}

fn check_winner(board: &Vec<Vec<usize>>) -> Option<usize> {
    let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
    for y in 0..DEFAULT_C4_ROWS {
        for x in 0..DEFAULT_C4_COLS {
            if board[y][x] != 0 {
                let current = board[y][x];
                for (dy, dx) in directions.iter() {
                    let mut count = 1;
                    let mut nx = x as isize + dx;
                    let mut ny = y as isize + dy;
                    while nx >= 0
                        && nx < DEFAULT_C4_COLS as isize
                        && ny >= 0
                        && ny < DEFAULT_C4_ROWS as isize
                        && board[ny as usize][nx as usize] == current
                    {
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

fn get_next_open_row(board: &Vec<Vec<usize>>, col: usize) -> Option<usize> {
    (0..DEFAULT_C4_ROWS).rev().find(|&row| board[row][col] == 0)
}

fn score_position(board: &Vec<Vec<usize>>, piece: usize) -> isize {
    let mut score = 0;
    let center_col = DEFAULT_C4_COLS / 2;

    let center_count = board
        .iter()
        .map(|row| (row[center_col] == piece) as isize)
        .sum::<isize>();
    score += center_count * 10;

    // Horizontal windows
    for row in board {
        for col in 0..=DEFAULT_C4_COLS - 4 {
            let window = &row[col..col + 4];
            score += evaluate_window(window, piece);
        }
    }

    // Vertical windows
    for col in 0..DEFAULT_C4_COLS {
        for row in 0..=DEFAULT_C4_ROWS - 4 {
            let window = (0..4).map(|i| board[row + i][col]).collect::<Vec<_>>();
            score += evaluate_window(&window, piece);
        }
    }

    // Positive Diagonal windows
    for row in 0..=DEFAULT_C4_ROWS - 4 {
        for col in 0..=DEFAULT_C4_COLS - 4 {
            let window = (0..4).map(|i| board[row + i][col + i]).collect::<Vec<_>>();
            score += evaluate_window(&window, piece);
        }
    }

    // Negative Diagonal windows
    for row in 3..DEFAULT_C4_ROWS {
        for col in 0..=DEFAULT_C4_COLS - 4 {
            let window = (0..4).map(|i| board[row - i][col + i]).collect::<Vec<_>>();
            score += evaluate_window(&window, piece);
        }
    }

    score
}

fn minimax(
    board: &Vec<Vec<usize>>,
    depth: usize,
    mut alpha: isize,
    mut beta: isize,
    is_maximizing: bool,
) -> (usize, isize) {
    if depth == 0 || check_winner(board).is_some() {
        return (0, score_position(board, COMPUTER));
    }

    if is_maximizing {
        let mut value = isize::MIN;
        let mut column = usize::MAX;
        for col in 0..DEFAULT_C4_COLS {
            if let Some(row) = get_next_open_row(board, col) {
                let mut temp_board = board.clone();
                temp_board[row][col] = COMPUTER;
                let new_score = minimax(&temp_board, depth - 1, alpha, beta, false).1;
                if new_score > value {
                    value = new_score;
                    column = col;
                }
                alpha = max(alpha, value);
                if alpha >= beta {
                    break;
                }
            }
        }
        (column, value)
    } else {
        let mut value = isize::MAX;
        let mut column = usize::MAX;
        for col in 0..DEFAULT_C4_COLS {
            if let Some(row) = get_next_open_row(board, col) {
                let mut temp_board = board.clone();
                temp_board[row][col] = USER;
                let new_score = minimax(&temp_board, depth - 1, alpha, beta, true).1;
                if new_score < value {
                    value = new_score;
                    column = col;
                }
                beta = min(beta, value);
                if alpha >= beta {
                    break;
                }
            }
        }
        (column, value)
    }
}

fn evaluate_window(window: &[usize], piece: usize) -> isize {
    let mut score = 0;
    let opp_piece = if piece == USER { COMPUTER } else { USER };
    let count_piece = window.iter().filter(|&&p| p == piece).count();
    let count_empty = window.iter().filter(|&&p| p == 0).count();
    let count_opp_piece = window.iter().filter(|&&p| p == opp_piece).count();

    match (count_piece, count_empty, count_opp_piece) {
        (4, 0, 0) => score += 10000,
        (0, 0, 4) => score -= 10000,
        (3, 1, 0) => score += 500,
        (0, 1, 3) => score -= 500,
        (2, 2, 0) => score += 50,
        (0, 2, 2) => score -= 50,
        _ => (),
    }

    score
}
