/// The grid of tiles in a game of minesweeper.
pub type Grid = Vec<Vec<Tile>>;

/// The location of a tile in the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

/// A tile in the game.
#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub is_bomb: bool,
    pub adj_bombs: u32,
    pub visibility: Visibility,
}

/// Is this tile revealed to the player?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Hidden,
    Revealed,
    Flagged,
}

impl Tile {
    /// Create a new tile, initially hidden.
    ///
    /// self.adj_bombs is *NOT* computed here.
    pub fn new(is_bomb: bool) -> Self {
        Self {
            is_bomb,
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
                if self.is_bomb {
                    'x'
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

/// After the game, did the user win or lose.
#[derive(Debug, Clone, Copy)]
pub enum GameOutcome {
    Win,
    Loss,
}
