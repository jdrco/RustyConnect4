use crate::constant::{DEFAULT_OT_COLS, DEFAULT_OT_ROWS, HEADER, RED_BAR};
use rand::prelude::*;
use std::cmp::{max, min};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, html};

#[function_component]
pub fn TootAndOttoBoard() -> Html {
    let board = use_state(|| vec![vec![(' ', 0); DEFAULT_OT_COLS]; DEFAULT_OT_ROWS]);
    let player_turn = use_state(|| 1);
    let player_choice = use_state(|| 'T');
    let winner = use_state(|| None::<usize>);
    let difficulty = use_state(|| "Easy".to_string());
    let player_t_count = use_state(|| 6);
    let player_o_count = use_state(|| 6);

    let handle_click = {
        let board = board.clone();
        let player_turn = player_turn.clone();
        let player_choice = player_choice.clone();
        let winner = winner.clone();
        let difficulty = difficulty.clone();
        let player_t_count = player_t_count.clone();
        let player_o_count = player_o_count.clone();
        Callback::from(move |x: usize| {
            if winner.is_none() {
                let mut new_board = (*board).clone();
                if let Some(y) = (0..DEFAULT_OT_ROWS)
                    .rev()
                    .find(|&y| new_board[y][x].0 == ' ')
                {
                    let current_choice = *player_choice;
                    let mut valid_move = false;

                    if current_choice == 'T' && *player_t_count > 0 {
                        player_t_count.set(*player_t_count - 1);
                        valid_move = true;
                    } else if current_choice == 'O' && *player_o_count > 0 {
                        player_o_count.set(*player_o_count - 1);
                        valid_move = true;
                    }

                    if valid_move {
                        new_board[y][x] = (current_choice, *player_turn);
                        if let Some(win_player) = check_winner(&new_board) {
                            winner.set(Some(win_player));
                        } else if is_full_board(&new_board)
                            || (*player_t_count == 0 && *player_o_count == 0)
                        {
                            winner.set(Some(3));
                        } else {
                            player_turn.set(3 - *player_turn);

                            if *difficulty == "Hard" {
                                make_computer_move(&mut new_board);
                            } else {
                                make_random_computer_move(&mut new_board);
                            }
                            if let Some(win_player) = check_winner(&new_board) {
                                winner.set(Some(win_player));
                            } else if is_full_board(&new_board)
                                || (*player_t_count == 0 && *player_o_count == 0)
                            {
                                winner.set(Some(3));
                            } else {
                                player_turn.set(1);
                            }
                        }
                        board.set(new_board);
                    }
                }
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

    let handle_difficulty_change = {
        let difficulty = difficulty.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            difficulty.set(input.value());
        })
    };

    html! {
        <div>
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
            <div class="post">
            <br/>
            <h4>{"Player Turn: "}{if *player_turn == 1 { "Player 1 (Red)" } else { "Player 2 (Yellow)" }}</h4>
            <small>{"Choose 'T' or 'O' to play."}</small>
            <br/>
            </div>
            <div id="gameboard" class="w-[500px] border border-black bg-boardPrimaryBg">
            { for (0..DEFAULT_OT_ROWS).map(|y| html! {
                    <div class="flex justify-center items-center gap-4 my-4">
                        { for (0..DEFAULT_OT_COLS).map(|x| html! {
                            <div onclick={handle_click.reform(move |_| x)}
                            class={
                                    let base_class = "w-12 h-12 rounded-full flex items-center justify-center text-xl text-black";
                                    let color_class = if board[y][x].1 == 1 {
                                        "bg-chipPrimaryBg"
                                    } else if board[y][x].1 == 2 {
                                        "bg-chipSecondaryBg"
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
                        <div>
                              {
                        if *player_t_count > 0 {
                            html! {
                                <div>
                                    <input type="radio" id="choose_t" name="player_choice" value="T"
                                           checked={*player_choice == 'T'}
                                           onchange={handle_option_change.clone()}/>
                                    <label for="choose_t">{"Choose T"}</label>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                    {
                        if *player_o_count > 0 {
                            html! {
                                <div>
                                    <input type="radio" id="choose_o" name="player_choice" value="O"
                                           checked={*player_choice == 'O'}
                                           onchange={handle_option_change}/>
                                    <label for="choose_o">{"Choose O"}</label>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                        </div>
            { if let Some(winner_player) = *winner {
                popup_modal(winner_player)
            } else {
                html! {}
            }}
        </div>

    }
}

fn check_winner(board: &Vec<Vec<(char, usize)>>) -> Option<usize> {
    let toot_sequence = ['T', 'O', 'O', 'T'];
    let otto_sequence = ['O', 'T', 'T', 'O'];
    let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
    let mut found_toot = false;
    let mut found_otto = false;

    for y in 0..DEFAULT_OT_ROWS {
        for x in 0..DEFAULT_OT_COLS {
            if board[y][x].0 != ' ' {
                for &(dy, dx) in &directions {
                    if check_sequence(board, x, y, dx, dy, &toot_sequence) {
                        found_toot = true;
                    }
                    if check_sequence(board, x, y, dx, dy, &otto_sequence) {
                        found_otto = true;
                    }
                }
            }
        }
    }

    match (found_toot, found_otto) {
        (true, false) => Some(1), // Player 1 wins with TOOT
        (false, true) => Some(2), // Player 2 wins with OTTO
        (true, true) => Some(3),  // Both sequences formed, possible in rare scenarios
        _ => None,
    }
}

fn check_sequence(
    board: &Vec<Vec<(char, usize)>>,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    sequence: &[char],
) -> bool {
    for (index, &item) in sequence.iter().enumerate() {
        let nx = x as isize + index as isize * dx;
        let ny = y as isize + index as isize * dy;

        if nx < 0
            || nx >= DEFAULT_OT_COLS as isize
            || ny < 0
            || ny >= DEFAULT_OT_ROWS as isize
            || board[ny as usize][nx as usize].0 != item
        {
            return false;
        }
    }
    true
}

fn is_full_board(board: &Vec<Vec<(char, usize)>>) -> bool {
    board.iter().all(|row| row.iter().all(|(c, _)| *c != ' '))
}

fn check_sequence_score(
    board: &Vec<Vec<(char, usize)>>,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    sequence: &[char],
    piece: char,
    win_score: isize,
    block_score: isize,
    advance_score: isize,
    block_advance_score: isize,
) -> isize {
    let mut score = 0;
    let mut match_count = 0;
    let mut empty_count = 0;

    for (index, &char) in sequence.iter().enumerate() {
        let nx = x as isize + index as isize * dx;
        let ny = y as isize + index as isize * dy;

        if nx < 0 || nx >= DEFAULT_OT_COLS as isize || ny < 0 || ny >= DEFAULT_OT_ROWS as isize {
            continue;
        }

        if board[ny as usize][nx as usize].0 == char {
            match_count += 1;
        } else if board[ny as usize][nx as usize].0 == ' ' {
            empty_count += 1;
        } else {
            return 0;
        }
    }

    if match_count == sequence.len() - 1 && empty_count == 1 {
        if piece == sequence[0] {
            score += win_score;
        } else {
            score -= block_score;
        }
    } else {
        score +=
            (match_count as isize * advance_score) - (empty_count as isize * block_advance_score);
    }

    score
}

fn evaluate_board(board: &Vec<Vec<(char, usize)>>, piece: char) -> isize {
    let mut score = 0;

    const WIN_SCORE: isize = 10000;
    const BLOCK_SCORE: isize = 15000; // Increased block score
    const ADVANCE_SCORE: isize = 100;
    const BLOCK_ADVANCE_SCORE: isize = 200; // Increased block advance score

    let otto = ['O', 'T', 'T', 'O'];
    let toot = ['T', 'O', 'O', 'T'];
    let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];

    for y in 0..DEFAULT_OT_ROWS {
        for x in 0..DEFAULT_OT_COLS {
            for &(dy, dx) in &directions {
                let otto_score = check_sequence_score(
                    board,
                    x,
                    y,
                    dx,
                    dy,
                    &otto,
                    piece,
                    WIN_SCORE,
                    BLOCK_SCORE,
                    ADVANCE_SCORE,
                    BLOCK_ADVANCE_SCORE,
                );
                let toot_score = check_sequence_score(
                    board,
                    x,
                    y,
                    dx,
                    dy,
                    &toot,
                    piece,
                    WIN_SCORE,
                    BLOCK_SCORE,
                    ADVANCE_SCORE,
                    BLOCK_ADVANCE_SCORE,
                );
                score += otto_score + toot_score;
            }
        }
    }

    score
}

fn minimax(
    board: &Vec<Vec<(char, usize)>>,
    depth: usize,
    alpha: isize,
    beta: isize,
    is_maximizing: bool,
) -> (usize, isize) {
    if depth == 0 || check_winner(board).is_some() {
        let eval_piece = if is_maximizing { 'O' } else { 'T' };
        let score = evaluate_board(board, eval_piece);
        println!("Leaf node score: {}, depth: {}", score, depth);
        return (0, score);
    }

    let mut alpha = alpha;
    let mut beta = beta;
    let mut best_col = usize::MAX;
    let mut value = if is_maximizing {
        isize::MIN
    } else {
        isize::MAX
    };

    let current_piece = if is_maximizing { 'O' } else { 'T' };

    for col in 0..DEFAULT_OT_COLS {
        if let Some(row) = (0..DEFAULT_OT_ROWS).rev().find(|&r| board[r][col].0 == ' ') {
            let mut temp_board = board.clone();
            temp_board[row][col] = (current_piece, if is_maximizing { 2 } else { 1 });

            let (_, new_score) = minimax(&temp_board, depth - 1, alpha, beta, !is_maximizing);

            if is_maximizing {
                if new_score > value {
                    value = new_score;
                    best_col = col;
                }
                alpha = max(alpha, value);
            } else {
                if new_score < value {
                    value = new_score;
                    best_col = col;
                }
                beta = min(beta, value);
            }

            if alpha >= beta {
                break;
            }
        }
    }
    (best_col, value)
}

fn make_computer_move(board: &mut Vec<Vec<(char, usize)>>) {
    let (col_o, score_o) = minimax(board, 6, isize::MIN, isize::MAX, true);
    let (col_t, score_t) = minimax(board, 6, isize::MIN, isize::MAX, false);

    let (best_piece, best_col) = if score_o >= score_t {
        ('O', col_o)
    } else {
        ('T', col_t)
    };

    if let Some(row) = (0..DEFAULT_OT_ROWS)
        .rev()
        .find(|&r| board[r][best_col].0 == ' ')
    {
        board[row][best_col] = (best_piece, 2);
    }
}

fn popup_modal(winner: usize) -> Html {
    html! {
        <div class={"modal fixed z-1 left-0 top-0 w-full h-full overflow-auto bg-black bg-opacity-40"}>
            <div class={"modal-content bg-gray-100 mx-auto my-15 p-5 border border-gray-400 w-4/5"}>
               {
                if winner == 1 {
                    html! {<h3>{"Player 1 Wins!"}</h3>}
                } else if winner == 2 {
                    html! {<h3>{"Player 2 Wins!"}</h3>}
                } else {
                    html! {<h3>{"It's a Draw!"}</h3>}
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

// Add code here

#[function_component]
pub fn TootAndOttoRules() -> Html {
    html! {
        <div id="main">
            <div class="container mx-auto mt-12" id="services">
                <h5 class={HEADER}><b>{"How to Play TOOT-OTTO"}</b></h5>
                <div class={RED_BAR}/>
                <p>{"TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players can place both T's and O's, based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}</p>
                <br/>
                <div><h5>{"To play TOOT-OTTO follow the following steps:"}</h5></div>
                <ul>
                    <li>{"A new game describes which player is TOOT and which is OTTO"}</li>
                    <li>{"Select the disc type T or O that you want to place"}</li>
                    <li>{"Click on the desired column on the game board to place your disc"}</li>
                    <li>{"Try to spell TOOT or OTTO based on your winning combination, either horizontally, vertically or diagonally"}</li>
                </ul>
                <br/>
                <p>{"For More information on TOOT-OTTO click "}<a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{"here"}</a></p>
            </div>
        </div>
    }
}

fn make_random_computer_move(board: &mut Vec<Vec<(char, usize)>>) {
    let mut rng = rand::thread_rng();
    let available_cols: Vec<usize> = (0..DEFAULT_OT_COLS)
        .filter(|&col| board[0][col].0 == ' ')
        .collect();

    if let Some(&col) = available_cols.choose(&mut rng) {
        if let Some(row) = (0..DEFAULT_OT_ROWS).rev().find(|&r| board[r][col].0 == ' ') {
            let computer_choice = if rng.gen_bool(0.5) { 'T' } else { 'O' };
            board[row][col] = (computer_choice, 2);
        }
    }
}
