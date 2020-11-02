use crate::game::{Game, Point, Visibility};
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Write};

/// An input command typed by the user.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Point(Point),
    Exit,
}

/// Prompt the user for their move.
///
/// Re-try if they type an malformed input, or an illegal move.
///
/// If a point is returned, it will be [`Visibility::Hidden`].
pub fn get_user_move(game: &Game) -> io::Result<Command> {
    loop {
        // Prompt.
        print!("> ");
        io::stdout().flush()?;

        match get_input()? {
            None => println!("Valid moves consist of two numbers: row, col"),
            Some(Command::Exit) => return Ok(Command::Exit),
            Some(Command::Point(p)) => match game.get(p).map(|tile| tile.visibility) {
                Some(Visibility::Hidden) => return Ok(Command::Point(p)),
                Some(Visibility::Revealed) => println!("Already revealed: ({}, {})", p.row, p.col),
                Some(Visibility::Flagged) => {
                    println!(
                        "Tile is flagged: ({}, {}). Unflag before proceeding.",
                        p.row, p.col
                    );
                }
                None => {
                    let (height, width) = game.dimensions();
                    println!(
                        "Indeces out of bounds: ({}, {}). Max tile is: ({}, {}).",
                        p.row,
                        p.col,
                        height - 1,
                        width - 1
                    );
                }
            },
        }
    }
}

/// Read a line of input from the user (i.e., stdin).
///
/// See `parse_user_input` for details.
fn get_input() -> io::Result<Option<Command>> {
    let mut line = String::new();
    if io::stdin().read_line(&mut line)? == 0 {
        // Input stream closed (i.e. ctrl-d at the command line).
        return Ok(Some(Command::Exit));
    }

    Ok(parse_input(&line))
}

/// Matches 2 non-negative numbers, or the single word "quit" (or "exit").
///
/// If the user typed a malformed input, return Ok(None).
fn parse_input(line: &str) -> Option<Command> {
    lazy_static! {
        static ref ALPHA_NUM: Regex = Regex::new(r"[\d\p{Alphabetic}]").unwrap();
        static ref ALPHABETIC: Regex = Regex::new(r"\p{Alphabetic}").unwrap();
        static ref QUIT_OR_EXIT: Regex = Regex::new(r"(?i)^(quit|exit)$").unwrap();
        static ref NUM: Regex = Regex::new(r"\d+").unwrap();
    }

    // Trim down to just the alpha-numeric characters (and anything between them).
    // E.g.: "(2, 2)" => "2, 2"
    //       "  quit. " => "quit"
    let not_alpha_num = |c| !ALPHA_NUM.is_match(&format!("{}", c));
    let trimmed = line.trim_matches(not_alpha_num);

    // Check for exactly "quit" or "exit".
    if QUIT_OR_EXIT.is_match(trimmed) {
        return Some(Command::Exit);
    }

    // There should be no letters in the input.
    // E.g.: "2 and 3" is invalid.
    if ALPHABETIC.is_match(trimmed) {
        return None;
    }

    // Find exactly 2 numbers (strings of contiguous digits).
    // We don't care what else is in the input; any separators are fine.
    // E.g.: "2, 2" => Point { row: 2, col: 2 }
    let mut matches = NUM.find_iter(trimmed);
    match (matches.next(), matches.next(), matches.next()) {
        (Some(m1), Some(m2), None) => match (m1.as_str().parse(), m2.as_str().parse()) {
            (Ok(row), Ok(col)) => Some(Command::Point(Point { row, col })),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_exit() {
        let exit = Some(Command::Exit);

        assert_eq!(parse_input("quit"), exit);
        assert_eq!(parse_input("exit"), exit);
        assert_eq!(parse_input("  quit "), exit);
        assert_eq!(parse_input("exit..."), exit);
        assert_eq!(parse_input("quit!"), exit);
    }

    #[test]
    fn parse_input_point() {
        let point = Some(Command::Point(Point { row: 6, col: 0 }));

        assert_eq!(parse_input("6 0"), point);
        assert_eq!(parse_input("(6, 0)"), point);
        assert_eq!(parse_input("6*00"), point);
        assert_eq!(parse_input("-006-00-"), point);
    }

    #[test]
    fn parse_input_error() {
        let error = None;
        assert_eq!(parse_input("asdf"), error);
        assert_eq!(parse_input(" exit quit"), error);
        assert_eq!(parse_input("6 and 0"), error);
    }
}
