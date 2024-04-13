use crate::constant::{DEFAULT_OT_COLS, DEFAULT_OT_ROWS, HEADER, RED_BAR};
use gloo_timers::callback::Timeout;
use rand::prelude::*;
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
    let last_move = use_state(|| None::<(usize, usize)>);
    let is_user_turn = use_state(|| true);
    let player_t_pieces = use_state(|| vec![6, 6]); // First idx is user, second is computer
    let player_o_pieces = use_state(|| vec![6, 6]);

    let handle_click = {
        let board = board.clone();
        let player_turn = player_turn.clone();
        let player_choice = player_choice.clone();
        let winner = winner.clone();
        let difficulty = difficulty.clone();
        let last_move = last_move.clone();
        let is_user_turn = is_user_turn.clone();
        let player_t_pieces = player_t_pieces.clone();
        let player_o_pieces = player_o_pieces.clone();

        Callback::from(move |x: usize| {
            if !*is_user_turn {
                return;
            }
            if winner.is_none() {
                let mut new_board = (*board).clone();
                let mut new_player_t_pieces = (*player_t_pieces).clone();
                let mut new_player_o_pieces = (*player_o_pieces).clone();

                // Check if the player has enough tokens
                let choice = *player_choice;
                let (player_tokens, opponent_tokens) = if choice == 'T' {
                    (&mut new_player_t_pieces, &mut new_player_o_pieces)
                } else {
                    (&mut new_player_o_pieces, &mut new_player_t_pieces)
                };

                if player_tokens[0] == 0 {
                    // If the player has run out of tokens, return without processing the click event
                    return;
                }

                if let Some(y) = (0..DEFAULT_OT_ROWS)
                    .rev()
                    .find(|&y| new_board[y][x].0 == ' ')
                {
                    new_board[y][x] = (choice, *player_turn);
                    last_move.set(Some((x, y)));
                    is_user_turn.set(false);

                    // Decrement the player's token count
                    player_tokens[0] -= 1;

                    board.set(new_board.clone());

                    if let Some(win_player) = check_winner(&new_board) {
                        winner.set(Some(win_player));
                    } else if is_full_board(&new_board) {
                        winner.set(Some(3));
                    } else {
                        let new_board = new_board.clone();
                        let last_move = last_move.clone();
                        let difficulty = difficulty.clone();
                        let winner = winner.clone();
                        let player_turn = player_turn.clone();
                        let is_user_turn = is_user_turn.clone();
                        let board = board.clone();
                        let new_player_t_pieces = new_player_t_pieces.clone();
                        let new_player_o_pieces = new_player_o_pieces.clone();
                        let player_t_pieces = player_t_pieces.clone();
                        let player_o_pieces = player_o_pieces.clone();
                        player_turn.set(2);

                        let timeout = Timeout::new(500, move || {
                            let difficulty = difficulty.clone();
                            let mut new_board = new_board;
                            let mut new_player_t_pieces = new_player_t_pieces;
                            let mut new_player_o_pieces = new_player_o_pieces;
                            if *difficulty == "Hard" {
                                make_computer_move(
                                    &mut new_board,
                                    2,
                                    &mut new_player_t_pieces,
                                    &mut new_player_o_pieces,
                                );
                            } else {
                                make_random_computer_move(
                                    &mut new_board,
                                    &mut new_player_t_pieces,
                                    &mut new_player_o_pieces,
                                );
                            }
                            last_move.set(Some((x, y)));
                            if let Some(win_player) = check_winner(&new_board) {
                                winner.set(Some(win_player));
                            } else if is_full_board(&new_board) {
                                winner.set(Some(3));
                            } else {
                                player_turn.set(1);
                            }
                            board.set(new_board);
                            player_t_pieces.set(new_player_t_pieces);
                            player_o_pieces.set(new_player_o_pieces);
                            is_user_turn.set(true);
                        });
                        timeout.forget();
                    }
                }
            }
        })
    };

    let handle_option_change = {
        let player_choice = player_choice.clone();
        let player_t_pieces = player_t_pieces.clone();
        let player_o_pieces = player_o_pieces.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let choice = input.value().chars().next().unwrap_or('T');

            if choice == 'T' && player_t_pieces[0] == 0 {
                // Change player's choice to 'O' if they run out of 'T' tokens
                player_choice.set('O');
            } else if choice == 'O' && player_o_pieces[0] == 0 {
                // Change player's choice to 'T' if they run out of 'O' tokens
                player_choice.set('T');
            } else {
                player_choice.set(choice);
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
            <div>
                <input type="radio" id="choose_t" name="player_choice" value="T"
                       checked={*player_choice == 'T'}
                       onchange={handle_option_change.clone()}
                       disabled={player_t_pieces[0] == 0}/>
                <label for="choose_t">{"Choose T"}</label>

                <input type="radio" id="choose_o" name="player_choice" value="O"
                       checked={*player_choice == 'O'}
                       onchange={handle_option_change}
                       disabled={player_o_pieces[0] == 0}/>
                <label for="choose_o">{"Choose O"}</label>
            </div>
            <div class="post">
                <br/>
                <h4>{"Player Turn: "}{if *player_turn == 1 { "Player 1 (Red)" } else { "Player 2 (Yellow)" }}</h4>
                <small>{"Choose 'T' or 'O' to play."}</small>
                <br/>
            </div>
            <div>
                <div>{"User has "}
                    {player_o_pieces[0]}{" 'O's left and "}
                    {player_t_pieces[0]}{" 'T's left"}
                </div>
                <div>{"Computer has "}
                    {player_o_pieces[1]}{" 'O's left and "}
                    {player_t_pieces[1]}{" 'T's left"}
                </div>
            </div>
            <div id="gameboard" class="w-[500px] border border-black bg-boardPrimaryBg">
                { for (0..DEFAULT_OT_ROWS).map(|y| html! {
                    <div class="flex justify-center items-center gap-4 my-4">
                        { for (0..DEFAULT_OT_COLS).map(|x| html! {
                            <div onclick={handle_click.reform(move |_| x)}
                                 class={
                                    let base_class = "w-12 h-12 rounded-full flex items-center justify-center text-xl text-black";
                                    let is_last_move = *last_move == Some((x, y));
                                    let animation_class = if is_last_move { "animate-drop" } else { "" };
                                    let color_class = if board[y][x].1 == 1 {
                                        "bg-chipPrimaryBg"
                                    } else if board[y][x].1 == 2 {
                                        "bg-chipSecondaryBg"
                                    } else {
                                        "bg-white"
                                    };
                                    format!("{} {} {}", base_class, color_class, animation_class)
                                 }>
                                { board[y][x].0.to_string() }
                            </div>
                        })}
                    </div>
                })}
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
    let otto = ['O', 'T', 'T', 'O'];
    let toot = ['T', 'O', 'O', 'T'];
    let mut score = 0;
    let mut match_count = 0;
    let mut empty_count = 0;
    let mut opponent_count = 0;

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
            opponent_count += 1;
        }
    }
    if opponent_count == 4 {
        score -= win_score * 2;
    } else {
        match match_count {
            4 => {
                score += win_score;
            }
            3 => {
                // OTT_ (Good) vs. OTTT (Bad)
                if empty_count == 1 {
                    score += advance_score * 5; // Increase score for winning sequences
                } else {
                    score -= block_score;
                }
            }
            2 => {
                // OT__, _TT_, O__O, __TO
                if empty_count == 2 {
                    score += advance_score; // Increase score for winning sequences
                } else if empty_count == 1 && opponent_count == 1 {
                    // OTO_
                    score -= block_advance_score * 4;
                } else {
                    // OTTT, OOTO
                    score -= block_advance_score * 4;
                }
            }
            1 => {
                // _T__, __T_, T___, T__T
                if empty_count == 3 {
                    score += advance_score;
                } else if empty_count == 2 && opponent_count == 1 {
                    // _TT_, T_TO, TO_T
                    score -= block_advance_score * 2;
                } else if empty_count == 1 && opponent_count == 3 {
                    // TTT_, _TTT, T_TT
                    score -= win_score * 2;
                } else if empty_count == 0 && opponent_count == 3 {
                    score += block_score;
                }
            }
            0 => {
                if empty_count == 4 {
                    score += advance_score;
                } else if opponent_count == 3 {
                    // TTT_, _TTT, T_TT
                    score -= block_score;
                } else if opponent_count == 4 {
                    score -= win_score * 3;
                }
            }
            _ => {}
        }
    }

    // // Penalize if placing the third 'O' or 'T' without blocking TOOT
    // if (piece == 'O' && opponent_count < 2 && sequence[0] == 'O')
    //     || (piece == 'T' && opponent_count < 2 && sequence[0] == 'T')
    // {
    //     score -= block_score;
    // }

    // // Award sequences with potential to form OTTO or TTOO
    // if match_count == 2 && empty_count == 2 && (piece == 'O' || piece == 'T') {
    //     if sequence == otto {
    //         score += win_score;
    //     }
    // }

    // // Award sequences of 4 that match the desired patterns
    // if match_count == 3 && empty_count == 1 {
    //     if (piece == 'O' && sequence[0] == 'O') || (piece == 'T' && sequence[0] == 'T') {
    //         score += advance_score;
    //     } else if piece == 'O' && sequence[3] == 'O' {
    //         score += advance_score;
    //     }
    // }

    // // Punish for three or more consecutive T's
    // if match_count >= 3 && piece == 'T' {
    //     score -= block_score;
    // }

    score
}

fn evaluate_board(board: &Vec<Vec<(char, usize)>>, piece: char) -> isize {
    let mut score = 0;

    const WIN_SCORE: isize = 100;
    const BLOCK_SCORE: isize = 90; // Increased block score
    const ADVANCE_SCORE: isize = 10;
    const BLOCK_ADVANCE_SCORE: isize = 10; // Increased block advance score

    let otto = ['O', 'T', 'T', 'O'];
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
                score += otto_score;
            }
        }
    }
    score
}

fn negamax(
    board: &Vec<Vec<(char, usize)>>,
    depth: usize,
    alpha: isize,
    beta: isize,
    current_player: usize,
    player_turn: usize,
    piece: char,
) -> (usize, isize) {
    if depth == 0 || check_winner(board).is_some() {
        let score = evaluate_board(board, piece);
        return (
            usize::MAX,
            if player_turn == current_player {
                score
            } else {
                -score
            },
        );
    }

    let mut alpha = alpha;
    let mut best_value = isize::MIN;
    let mut best_col = usize::MAX;

    for col in 0..DEFAULT_OT_COLS {
        if let Some(row) = (0..DEFAULT_OT_ROWS).rev().find(|&r| board[r][col].0 == ' ') {
            let mut new_board = board.clone();
            new_board[row][col] = (piece, player_turn);

            // Log the row and piece being checked
            web_sys::console::log_1(
                &format!("Checking row {} at col {} with piece {}", row, col, piece).into(),
            );

            for &next_piece in &['T', 'O'] {
                let (_, value) = negamax(
                    &new_board,
                    depth - 1,
                    beta.wrapping_neg(),
                    alpha.wrapping_neg(),
                    3 - current_player,
                    player_turn,
                    next_piece,
                );

                let value = -value;
                if value > best_value {
                    best_value = value;
                    best_col = col; // Update best_col here
                }
                alpha = alpha.max(value);
                if alpha >= beta {
                    break; // Beta cut-off
                }
            }
        }
    }

    web_sys::console::log_1(
        &format!(
            "Best col: {}, Best score: {}, Checking piece: {}",
            best_col, best_value, piece,
        )
        .into(),
    );

    (best_col, best_value)
}

fn make_computer_move(
    board: &mut Vec<Vec<(char, usize)>>,
    player_turn: usize,
    player_t_pieces: &mut Vec<i32>,
    player_o_pieces: &mut Vec<i32>,
) {
    let mut best_col = usize::MAX;
    let mut best_value = isize::MIN;
    let mut best_piece = 'T';

    for &current_piece in &['T', 'O'] {
        let (col, value) = negamax(
            board,
            5,
            isize::MIN,
            isize::MAX,
            player_turn,
            player_turn,
            current_piece,
        );
        if value > best_value && col < DEFAULT_OT_COLS {
            best_value = value;
            best_col = col;
            best_piece = current_piece;
        }
    }

    if best_col <= DEFAULT_OT_COLS {
        if let Some(row) = (0..DEFAULT_OT_ROWS)
            .rev()
            .find(|&r| board[r][best_col].0 == ' ')
        {
            if best_piece == 'T' {
                if player_t_pieces[1] <= 0 {
                    return;
                }
                player_t_pieces[1] -= 1;
            } else {
                if player_o_pieces[1] <= 0 {
                    return;
                }
                player_o_pieces[1] -= 1;
            }
            board[row][best_col] = (best_piece, player_turn);
        }
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
                <p>{"TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players can place both SIX T's and O's , based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}</p>
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

fn make_random_computer_move(
    board: &mut Vec<Vec<(char, usize)>>,
    player_t_pieces: &mut Vec<i32>,
    player_o_pieces: &mut Vec<i32>,
) {
    let mut rng = rand::thread_rng();
    let available_cols: Vec<usize> = (0..DEFAULT_OT_COLS)
        .filter(|&col| board[0][col].0 == ' ')
        .collect();

    if let Some(&col) = available_cols.choose(&mut rng) {
        if let Some(row) = (0..DEFAULT_OT_ROWS).rev().find(|&r| board[r][col].0 == ' ') {
            let computer_choice = if rng.gen_bool(0.5) { 'T' } else { 'O' };
            if computer_choice == 'T' {
                if player_t_pieces[1] <= 0 {
                    return;
                }
                player_t_pieces[1] -= 1;
            } else {
                if player_o_pieces[1] <= 0 {
                    return;
                }
                player_o_pieces[1] -= 1;
            }
            board[row][col] = (computer_choice, 2);
        }
    }
}
