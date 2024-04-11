use crate::constant::{DEFAULT_OT_COLS, DEFAULT_OT_ROWS, HEADER, RED_BAR};
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

    let handle_click = {
        let board = board.clone();
        let player_turn = player_turn.clone();
        let player_choice = player_choice.clone();
        let winner = winner.clone();
        Callback::from(move |x: usize| {
            if winner.is_none() {
                let mut new_board = (*board).clone();
                if let Some(y) = (0..DEFAULT_OT_ROWS)
                    .rev()
                    .find(|&y| new_board[y][x].0 == ' ')
                {
                    new_board[y][x] = (*player_choice, *player_turn);
                    if let Some(win_player) = check_winner(&new_board) {
                        winner.set(Some(win_player));
                    } else if is_full_board(&new_board) {
                        winner.set(Some(3));
                    } else {
                        player_turn.set(2);
                        make_computer_move(&mut new_board);
                        if let Some(win_player) = check_winner(&new_board) {
                            winner.set(Some(win_player));
                        } else if is_full_board(&new_board) {
                            winner.set(Some(3));
                        } else {
                            player_turn.set(1);
                        }
                    }
                    board.set(new_board);
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

    html! {
        <div>
            <div class={HEADER}><b>{"Enter Your Name"}</b></div>
            <div class={RED_BAR}></div>
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
            { if let Some(winner_player) = *winner {
                popup_modal(winner_player)
            } else {
                html! {}
            }}
        </div>

    }
}

fn make_computer_move(board: &mut Vec<Vec<(char, usize)>>) {
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

fn check_winner(board: &Vec<Vec<(char, usize)>>) -> Option<usize> {
    let toot_sequence = ['T', 'O', 'O', 'T'];
    let otto_sequence = ['O', 'T', 'T', 'O'];
    let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];

    for y in 0..DEFAULT_OT_ROWS {
        for x in 0..DEFAULT_OT_COLS {
            if board[y][x].0 != ' ' {
                for &(dy, dx) in &directions {
                    if check_sequence(board, x, y, dx, dy, &toot_sequence) {
                        return Some(1);
                    }
                    if check_sequence(board, x, y, dx, dy, &otto_sequence) {
                        return Some(2);
                    }
                }
            }
        }
    }
    None
}

fn check_sequence(
    board: &Vec<Vec<(char, usize)>>,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    sequence: &[char],
) -> bool {
    for (index, &char) in sequence.iter().enumerate() {
        let nx = x as isize + index as isize * dx;
        let ny = y as isize + index as isize * dy;

        if nx < 0
            || nx >= DEFAULT_OT_COLS as isize
            || ny < 0
            || ny >= DEFAULT_OT_ROWS as isize
            || board[ny as usize][nx as usize].0 != char
        {
            return false;
        }
    }
    true
}

fn is_full_board(board: &Vec<Vec<(char, usize)>>) -> bool {
    board.iter().all(|row| row.iter().all(|(c, _)| *c != ' '))
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
                    <li>{"Try to spell TOOT or OTTO based on your winning combination, either horizontally or vertically or diagonally"}</li>
                </ul>
                <br/>
                <p>{"For More information on TOOT-OTTO click "}<a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{"here"}</a></p>
            </div>
        </div>
    }
}
