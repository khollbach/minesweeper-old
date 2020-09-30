use std::io::{self, BufRead, BufReader, Read};

/// A tile in the game.
pub struct Tile {
    has_bomb: bool,
    adj_bombs: u32,
    visibility: Visibility,
}

/// Is this tile revealed to the player?
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

/// The grid of tiles in the game.
pub struct Grid(Vec<Vec<Tile>>);

impl Grid {
    /// New-line separated rows of chars.
    ///
    /// See `Tile.to_char` for more info.
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for row in &self.0 {
            for tile in row {
                s.push(tile.to_char());
            }
            s.push('\n');
        }
        s
    }

    /// Mark all tiles as revealed; e.g. when the game ends.
    ///
    /// todo: do we want to still show flags when the game ends ?
    pub fn reveal_all(&mut self) {
        for row in &mut self.0 {
            for tile in row {
                tile.visibility = Visibility::Revealed;
            }
        }
    }
}

/// Parse an input stream into a Grid.
///
/// Valid input chars are '#' or '.' for bomb / empty. Otherwise, this will fail.
pub fn parse<R: Read>(input: BufReader<R>) -> io::Result<Grid> {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line?.chars() {
            let has_bomb = match c {
                '#' => true,
                '.' => false,
                _ => {
                    let err = io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Unexpected char in grid input: {}", c),
                    );
                    return Err(err);
                }
            };
            row.push(Tile::new(has_bomb));
        }
        grid.push(row);
    }

    let mut grid = Grid(grid);
    compute_adj_bombs(&mut grid);
    Ok(grid)
}

/// Populate the tiles with their correct `.adj_bombs` values.
fn compute_adj_bombs(grid: &mut Grid) {
    for i in 0..grid.0.len() {
        for j in 0..grid.0[i].len() {
            grid.0[i][j].adj_bombs = adj_bombs(grid, i, j);
        }
    }
}

/// How many bombs are adjacent to this tile? Indeces must be in range.
///
/// (Does not include this tile itself, if its a bomb.)
fn adj_bombs(grid: &Grid, i: usize, j: usize) -> u32 {
    let i = i as isize;
    let j = j as isize;

    let n = grid.0.len() as isize;
    let m = grid.0[0].len() as isize;
    debug_assert!((0..n).contains(&i) && (0..m).contains(&j));

    // Check all 8 adj tiles for bombs.
    let mut bombs = 0;
    for &di in &[-1, 0, 1] {
        for &dj in &[-1, 0, 1] {
            let x = i + di;
            let y = j + dj;
            if (di, dj) != (0, 0)
                && (0..n).contains(&x)
                && (0..m).contains(&y)
                && grid.0[x as usize][y as usize].has_bomb
            {
                bombs += 1;
            }
        }
    }
    bombs
}

#[cfg(test)]
mod tests {
    use super::*;

    // todo add a couple more test files, including trivial (empty) one.
    #[test]
    fn io_small() {
        let mut grid = parse(BufReader::new(
            include_str!("../tests/io/small.in").as_bytes(),
        ))
        .unwrap();
        grid.reveal_all();

        let expected = include_str!("../tests/io/small.out");
        let actual = grid.to_string();
        assert_eq!(expected, actual);
    }
}
