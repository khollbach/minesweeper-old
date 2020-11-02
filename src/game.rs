use grid::{parse_grid, ParseResult};
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, Read};

mod grid;

pub use grid::{Grid, Point, Tile, Visibility};

/// After the game, did the user win or lose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// Read a grid from an input file. See [`parse_grid`] for details.
    pub fn from_file(file: File) -> io::Result<Game> {
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

        if tile.is_bomb {
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
        for row in self.grid.iter_mut() {
            for tile in row {
                tile.visibility = Visibility::Revealed;
            }
        }
        self.num_revealed = self.num_tiles();
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}
