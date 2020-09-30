use std::fs::File;
use std::io::{self, BufReader, Read};

mod parse_grid;

use parse_grid::{Grid, ParseResult};

/// The location of a tile in the grid.
#[derive(Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

/// A tile in the game.
#[derive(Debug)]
pub struct Tile {
    has_bomb: bool,
    adj_bombs: u32,
    visibility: Visibility,
}

/// Is this tile revealed to the player?
#[derive(Debug)]
pub enum Visibility {
    Hidden,
    Flagged,
    Revealed,
}

impl Tile {
    /// Create a new tile, initially hidden.
    ///
    /// self.adj_bombs is *NOT* computed here.
    pub fn new(has_bomb: bool) -> Self {
        Self {
            has_bomb,
            adj_bombs: 0,
            visibility: Visibility::Hidden,
        }
    }

    /// What should we display to the user?
    ///
    /// Depends on whether the tile is revealed/flagged, how many adjacent bombs, etc.
    pub fn to_char(&self) -> char {
        use Visibility::*;
        match self.visibility {
            Hidden => '.',
            Flagged => '*',
            Revealed => {
                if self.has_bomb {
                    '#'
                } else if self.adj_bombs == 0 {
                    ' '
                } else {
                    assert!(self.adj_bombs < 10);
                    self.adj_bombs.to_string().chars().next().unwrap()
                }
            }
        }
    }
}

/// The state of a game of minesweeper.
///
/// Mostly just the grid, plus some other data.
#[derive(Debug)]
pub struct Game {
    num_bombs: u32,
    grid: Grid,
}

impl Game {
    /// Read a grid from an input stream. See `parse_grid::parse` for more.
    pub fn from_input<R: Read>(input: &mut BufReader<R>) -> io::Result<Game> {
        let ParseResult { grid, num_bombs } = parse_grid::parse(input)?;
        Ok(Game { num_bombs, grid })
    }

    /// Read a grid from an input file. See `from_input` for more.
    pub fn from_file(file_name: &str) -> io::Result<Game> {
        let file = File::open(file_name)?;
        Game::from_input(&mut BufReader::new(file))
    }

    /// Newline-separated rows of chars. No trailing newline.
    ///
    /// See `Tile.to_char` for more info.
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for (i, row) in self.grid.iter().enumerate() {
            if i != 0 {
                s.push('\n');
            }
            for tile in row {
                s.push(tile.to_char());
            }
        }
        s
    }

    /// Mark all tiles as revealed; e.g. when the game ends.
    ///
    /// todo: do we want to still show flags when the game ends ?
    pub fn reveal_all(&mut self) {
        for row in &mut self.grid {
            for tile in row {
                tile.visibility = Visibility::Revealed;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod io {
        use super::*;
        use std::fs;

        mod good {
            use super::*;

            /// Parse the test input into a `Game`, and then turn the game back into a string.
            ///
            /// Check that the output matches the expected test output.
            fn good(test_name: &'static str) {
                let repo_root = env!("CARGO_MANIFEST_DIR");
                let path = format!("{}/tests/io/good/{}", repo_root, test_name);

                let mut game = Game::from_file(&format!("{}.in", path)).unwrap();
                game.reveal_all();
                let mut actual = game.to_string();
                actual.push('\n');

                let expected = fs::read_to_string(format!("{}.out", path)).unwrap();
                assert_eq!(expected, actual);
            }

            #[test]
            fn height_one() {
                good("height_one");
            }

            #[test]
            fn small() {
                good("small");
            }

            #[test]
            fn width_one() {
                good("width_one");
            }
        }

        mod bad {
            use super::*;

            /// Try to parse the test input, and ensure that doing so causes an Err.
            fn bad(test_name: &'static str) {
                let repo_root = env!("CARGO_MANIFEST_DIR");
                let path = format!("{}/tests/io/bad/{}", repo_root, test_name);

                let result = Game::from_file(&format!("{}.in", path));
                assert!(result.is_err());
            }

            #[test]
            fn height_zero() {
                bad("height_zero");
            }

            #[test]
            fn invalid_chars() {
                bad("invalid_chars");
            }

            #[test]
            fn jagged() {
                bad("jagged");
            }

            #[test]
            fn width_zero() {
                bad("width_zero");
            }
        }
    }
}
