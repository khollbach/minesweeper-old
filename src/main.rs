use game::{Game, GameOutcome};
use std::env;
use std::error::Error;
use user_input::{get_user_move, Command};

mod game;
mod user_input;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = parse_args()?;
    let mut game = Game::from_file(&file_name)?;

    loop {
        // Display grid.
        clear_screen();
        println!("{}", game.to_string());

        // Get input.
        let point_to_reveal = match get_user_move(&game)? {
            Command::Point(p) => p,
            Command::Exit => return Ok(()),
        };

        // Update game state.
        game.reveal(point_to_reveal);

        // Check if the game is over.
        match game.is_game_over() {
            Some(GameOutcome::Win) => {
                println!("Yay! You win!");
                return Ok(());
            }
            Some(GameOutcome::Loss) => {
                println!("Aww... you lost :(");
                return Ok(());
            }
            None => (),
        }
    }
}

/// Read the input file name from the commandline; else return a default.
fn parse_args() -> Result<String, Box<dyn Error>> {
    let mut args = env::args();

    // Ignore executable name.
    args.next().unwrap();

    let file_name = match args.next() {
        Some(f) => f,
        None => String::from("tests/io/good/small.in"),
    };

    if let Some(_) = args.next() {
        Err("Too many arguments. Expected just one filename.")?;
    }

    Ok(file_name)
}

/// Poor man's clear-screen; just print 100 newlines.
fn clear_screen() {
    //let n = 100;
    let n = 1; // for now, while I code
    for _ in 0..n {
        println!();
    }
}
