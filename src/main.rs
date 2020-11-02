use game::{Game, GameOutcome};
use std::env;
use std::error::Error;
use std::fs::File;
use user_input::{get_user_move, Command};

mod game;
mod user_input;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the game.
    let file_name = parse_args()?;
    let file = match File::open(&file_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", file_name);
            return Err(e.into());
        }
    };
    let mut game = Game::from_file(file)?;

    loop {
        // Display grid.
        clear_screen();
        println!("{}", game);

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
fn parse_args() -> Result<String, String> {
    let mut args = env::args();

    // Ignore executable name.
    args.next().unwrap();

    let file_name = match args.next() {
        Some(f) => f,
        None => {
            // Note this will only work on the machine where this code is compiled.
            // If you re-distribute the binary, the example input file won't be found.
            let repo_root = env!("CARGO_MANIFEST_DIR");
            let default = format!("{}/tests/parse_grid/good/small.in", repo_root);
            default
        }
    };

    // Too many args.
    if args.next().is_some() {
        return Err("Too many arguments. Expected just one filename.".into());
    }

    Ok(file_name)
}

/// Simple, hacky clear-screen: just print 100 newlines.
fn clear_screen() {
    //let n = 100;
    let n = 1; // for now, while I code
    for _ in 0..n {
        println!();
    }
}
