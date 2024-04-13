const DEFAULT_C4_ROWS: usize = 6;
const DEFAULT_C4_COLS: usize = 7;
const USER: usize = 1;
const COMPUTER: usize = 2;
const EMPTY: usize = 0;

use crate::components::connect4board;


// cli.rs
pub fn connect4() {
    let mut board = init_board();
    let mut current_player = USER; // Start with the user

    loop {
        display_board(&board);
        let column = get_player_input();
        
        if let Some(row) = get_next_open_row(&board, column) {
            board[row][column] = current_player;
            if let Some(winner) = connect4board::check_winner(&board) {
                display_board(&board);
                println!("Player {} wins!", winner);
                break;
            }
        } else {
            println!("Column is full or invalid, try another one.");
            continue;
        }

        current_player = if current_player == USER { COMPUTER } else { USER };
        
        // Handle computer move
        if current_player == COMPUTER {
            make_computer_move(&mut board);
            if let Some(winner) = connect4board::check_winner(&board) {
                display_board(&board);
                println!("Computer wins!");
                break;
            }
            current_player = USER; // Switch back to user
        }
    }
}


fn init_board() -> Vec<Vec<usize>> {
    vec![vec![EMPTY; DEFAULT_C4_COLS]; DEFAULT_C4_ROWS]
}

fn display_board(board: &Vec<Vec<usize>>) {
    for row in board {
        for &cell in row {
            let symbol = match cell {
                USER => "X",
                COMPUTER => "O",
                _ => ".",
            };
            print!("{} ", symbol);
        }
        println!();
    }
}

fn get_player_input() -> usize {
    println!("Enter a column number (0 to {}):", DEFAULT_C4_COLS - 1);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<usize>().unwrap_or(usize::MAX) // returns usize::MAX on parse failure
}

fn get_next_open_row(board: &Vec<Vec<usize>>, col: usize) -> Option<usize> {
    if col >= DEFAULT_C4_COLS {
        return None;
    }
    for row in (0..DEFAULT_C4_ROWS).rev() {
        if board[row][col] == EMPTY {
            return Some(row);
        }
    }
    None
}

pub fn make_computer_move(board: &mut Vec<Vec<usize>>) {

    let (best_col, _) = connect4board::minimax(board, 4, isize::MIN, isize::MAX, true);
    if let Some(row) = get_next_open_row(board, best_col) {
        println!("Computer picked column: {}", best_col);
        board[row][best_col] = COMPUTER;
    }
}