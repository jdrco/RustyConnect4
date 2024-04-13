mod components;
mod constant;
mod cli;

use crate::components::app::App;
use std::{io, process::{self, exit}};

fn main() {

    println!("Would you like to enable debugging? (y/n)");
    let mut debug_input = String::new();
    io::stdin().read_line(&mut debug_input).expect("Failed to read line");
    debug_input = debug_input.trim().to_lowercase();

    if debug_input == "y" {
        println!("Would game would you like to debug? (A: Connect4 / B: TOOT&OTTO)");
        let mut game_choice = String::new();
        io::stdin().read_line(&mut game_choice).expect("Failed to read line");
        let game_choice = game_choice.trim().to_lowercase();
        if game_choice == "a" {
            // Run Connect4
            cli::connect4();
        } else if game_choice == "b" {
            // Run Toot & Otto
        }
        process::exit(1);
    }

    yew::Renderer::<App>::new().render();
}
