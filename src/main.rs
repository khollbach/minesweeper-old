use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::error::Error;
use std::io::{self, Write};

mod game;

use game::{Game, GameOutcome, Point, Visibility};

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = read_args()?;
    let mut game = Game::from_file(&file_name)?;

    loop {
        // Display grid.
        clear_screen();
        println!("{}", game.to_string());

        // Get input.
        let point_to_reveal = loop {
            // Prompt.
            print!("> ");
            io::stdout().flush()?;

            use Input::*;
            use Visibility::*;
            match read_input()? {
                Point(p) => match game.get(p) {
                    Some(t) => match t.visibility {
                        Revealed => {
                            println!("Already revealed: ({}, {})", p.row, p.col);
                            continue;
                        }
                        Flagged => {
                            println!(
                                "Tile is flagged: ({}, {}). Unflag before proceeding.",
                                p.row, p.col
                            );
                            continue;
                        }
                        Hidden => break p,
                    },
                    None => {
                        let (height, width) = game.dimensions();
                        println!(
                            "Indeces out of bounds: ({}, {}). Max tile is: ({}, {}).",
                            p.row,
                            p.col,
                            height - 1,
                            width - 1
                        );
                        continue;
                    }
                },
                Exit => return Ok(()),
                Malformed => {
                    println!("Valid moves consist of two numbers: row, col");
                    continue;
                }
            }
        };

        // Update game state.
        game.reveal(point_to_reveal);

        // Check if the game is over.
        use GameOutcome::*;
        match game.game_over() {
            Some(Win) => {
                println!("Yay! You win!");
                return Ok(());
            }
            Some(Lose) => {
                println!("Aww... you lost :(");
                return Ok(());
            }
            None => (),
        }
    }
}

/// Read the input file name from the commandline; else return a default.
fn read_args() -> Result<String, Box<dyn Error>> {
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

/// A line of input typed by the user.
enum Input {
    Point(Point),
    Exit,
    Malformed,
}

/// Read a line of input from the user (stdin).
///
/// Look either for the word "quit" or "exit", or for 2 non-negative numbers.
fn read_input() -> io::Result<Input> {
    lazy_static! {
        static ref ALPHA_NUM: Regex = Regex::new(r"[\d\p{Alphabetic}]").unwrap();
        static ref QUIT_OR_EXIT: Regex = Regex::new(r"(?i)^(quit|exit)$").unwrap();
        static ref ALPHABETIC: Regex = Regex::new(r"\p{Alphabetic}").unwrap();
        static ref NUM: Regex = Regex::new(r"\d+").unwrap();
    }
    let not_alpha_num = |c| !ALPHA_NUM.is_match(&format!("{}", c));

    let mut line = String::new();
    if io::stdin().read_line(&mut line)? == 0 {
        // Input stream closed.
        return Ok(Input::Exit);
    }

    // Check for exactly "quit" or "exit".
    let trimmed = line.trim_matches(not_alpha_num);
    if QUIT_OR_EXIT.is_match(trimmed) {
        return Ok(Input::Exit);
    }

    // Are there are letters in the input?
    if ALPHABETIC.is_match(trimmed) {
        return Ok(Input::Malformed);
    }

    // Find exactly 2 numbers (strings of contiguous digits).
    // We don't care what else is in the input; any separators are fine.
    let mut matches = NUM.find_iter(trimmed);
    match (matches.next(), matches.next(), matches.next()) {
        (Some(m1), Some(m2), None) => match (m1.as_str().parse(), m2.as_str().parse()) {
            (Ok(row), Ok(col)) => Ok(Input::Point(Point { row, col })),
            _ => Ok(Input::Malformed),
        },
        _ => Ok(Input::Malformed),
    }
}
