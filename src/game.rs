use std::fs::File;
use std::io::{self, BufReader, Read};

mod parse_grid;

use parse_grid::{Grid, ParseResult};

/// The location of a tile in the grid.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

/// A tile in the game.
#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub has_bomb: bool,
    pub adj_bombs: u32,
    pub visibility: Visibility,
}

/// Is this tile revealed to the player?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Has the game finished?
#[derive(Debug, Clone, Copy)]
pub enum GameOutcome {
    Win,
    Loss,
}

/// The state of a game of minesweeper.
///
/// Mostly just the grid, plus some other data.
#[derive(Debug)]
pub struct Game {
    /// The grid of tiles. Guaranteed to be a non-empty rectangle.
    grid: Grid,
    /// The total number of bombs in the game.
    num_bombs: u32,
    /// The total number of *NON-BOMB* tiles successfully revealed by the player.
    num_revealed: u32,
    /// Is the game over, and how did it end?
    game_over: Option<GameOutcome>,
}

impl Game {
    /// Read a grid from an input stream. See `parse_grid::parse` for more.
    pub fn from_input<R: Read>(input: &mut BufReader<R>) -> io::Result<Game> {
        let ParseResult { grid, num_bombs } = parse_grid::parse(input)?;
        Ok(Game {
            grid,
            num_bombs,
            num_revealed: 0,
            game_over: None,
        })
    }

    /// Read a grid from an input file. See `from_input` for more.
    pub fn from_file(file_name: &str) -> io::Result<Game> {
        let file = File::open(file_name)?;
        Game::from_input(&mut BufReader::new(file))
    }

    /// Get the total number of bombs in the game.
    pub fn num_bombs(&self) -> u32 {
        self.num_bombs
    }

    /// Get the dimensions of the game grid: (height, width).
    pub fn dimensions(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    /// How many total tiles are there on the grid?
    pub fn num_tiles(&self) -> u32 {
        let (h, w) = self.dimensions();
        (h * w) as u32
    }

    /// Get a tile, or return None if the indeces are out of bounds.
    pub fn get(&self, point: Point) -> Option<Tile> {
        let Point { row, col } = point;
        if row < self.grid.len() && col < self.grid[row].len() {
            Some(self.grid[row][col])
        } else {
            None
        }
    }

    /// Reveal a tile.
    ///
    /// `point` must be in-range.
    pub fn reveal(&mut self, point: Point) {
        let Point { row, col } = point;
        let tile = &mut self.grid[row][col];

        if tile.visibility != Visibility::Revealed {
            tile.visibility = Visibility::Revealed;
            self.num_revealed += 1;
        }

        if tile.has_bomb {
            self.game_over = Some(GameOutcome::Loss)
        } else if self.num_revealed == self.num_tiles() - self.num_bombs {
            self.game_over = Some(GameOutcome::Win)
        }
    }

    /// Is the game over, and how did it end?
    ///
    /// Return None if the game is in progress.
    #[must_use]
    pub fn game_over(&self) -> Option<GameOutcome> {
        self.game_over
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
