use parse_grid::{parse_grid, ParseResult};
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, Read};

mod parse_grid;
mod types;

pub use types::{GameOutcome, Grid, Point, Tile, Visibility};

/// The state of a game of minesweeper.
///
/// Mostly just the grid, plus some other data.
#[derive(Debug)]
pub struct Game {
    /// The grid of tiles. Guaranteed to be a non-empty rectangle.
    grid: Grid,

    /// The total number of bombs in the game.
    num_bombs: u32,

    /// The total number of tiles revealed.
    num_revealed: u32,

    /// Is the game over, and how did it end?
    is_game_over: Option<GameOutcome>,
}

impl Game {
    /// Read a grid from an input stream. See [`parse_grid`] for details.
    pub fn from_input(input: &mut BufReader<impl Read>) -> io::Result<Game> {
        let ParseResult { grid, num_bombs } = parse_grid(input)?;

        Ok(Game {
            grid,
            num_bombs,
            num_revealed: 0,
            is_game_over: None,
        })
    }

    /// Read a grid from an input file. See [`Game::from_input`] for details.
    pub fn from_file(file_name: &str) -> io::Result<Game> {
        let file = File::open(file_name)?;

        Game::from_input(&mut BufReader::new(file))
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

    /// Get the total number of bombs in the game.
    pub fn num_bombs(&self) -> u32 {
        self.num_bombs
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

    /// Is the game over, and how did it end?
    ///
    /// Return None if the game is in progress.
    pub fn is_game_over(&self) -> Option<GameOutcome> {
        self.is_game_over
    }

    /// Reveal a tile. If this ends the game, update `self.is_game_over`.
    ///
    /// `point` must be in-range. The tile *must* be [`Visibility::Hidden`].
    pub fn reveal(&mut self, point: Point) {
        let Point { row, col } = point;
        let tile = &mut self.grid[row][col];

        assert_eq!(tile.visibility, Visibility::Hidden);

        tile.visibility = Visibility::Revealed;
        self.num_revealed += 1;

        if tile.has_bomb {
            // Hit a bomb -> you lose.
            self.is_game_over = Some(GameOutcome::Loss)
        } else {
            // Reveal all the non-bombs -> you win.
            let non_bombs = self.num_tiles() - self.num_bombs;
            if self.num_revealed >= non_bombs {
                assert_eq!(self.num_revealed, non_bombs);
                self.is_game_over = Some(GameOutcome::Win)
            }
        }
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
        self.num_revealed = self.num_tiles();
    }
}

impl fmt::Display for Game {
    /// Rows of the grid, with no trailing newline.
    ///
    /// See [`Tile::to_char`] for more details.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();

        for (i, row) in self.grid.iter().enumerate() {
            if i != 0 {
                buf.push('\n');
            }

            for tile in row {
                buf.push(tile.to_char());
            }
        }

        write!(f, "{}", buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod grid_parsing {
        use super::*;
        use std::fs;

        #[test]
        fn test_good_examples() {
            good("small");
            good("height_one");
            good("width_one");
            good("one_by_one_bomb");
            good("one_by_one_empty");
        }

        #[test]
        fn test_bad_examples() {
            bad("height_zero");
            bad("invalid_chars");
            bad("jagged");
            bad("width_zero");
        }

        /// Parse an input file into a `Game`, print the game to a string, and check that
        /// string against the output file.
        fn good(test_name: &'static str) {
            let repo_root = env!("CARGO_MANIFEST_DIR");
            let path = format!("{}/tests/grid-parsing/good/{}", repo_root, test_name);

            let mut game = Game::from_file(&format!("{}.in", path)).unwrap();
            game.reveal_all();

            let mut actual = game.to_string();
            actual.push('\n');

            let expected = fs::read_to_string(format!("{}.out", path)).unwrap();
            assert_eq!(expected, actual);
        }

        /// Try to parse a game grid, and unwrap an Err.
        fn bad(test_name: &'static str) {
            let repo_root = env!("CARGO_MANIFEST_DIR");
            let path = format!("{}/tests/grid-parsing/bad/{}", repo_root, test_name);

            let result = Game::from_file(&format!("{}.in", path));
            assert!(result.is_err());
        }
    }
}
