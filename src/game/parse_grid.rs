//! Grid-parsing logic.
//!
//! Tested in `game` module against grid-printing logic.

use std::io::{self, BufRead, BufReader, Read};

use crate::game::Tile;

/// The grid of tiles in a game of minesweeper.
pub type Grid = Vec<Vec<Tile>>;

/// Return value of `parse`.
pub struct ParseResult {
    pub grid: Grid,
    pub num_bombs: u32,
}

/// Parse an input stream into a Grid. The grid must be non-empty and rectangular.
///
/// Valid input chars are '#' or '.' for bomb / empty. Otherwise, this will fail.
pub fn parse<R: Read>(input: &mut BufReader<R>) -> io::Result<ParseResult> {
    let err = |s| Err(io::Error::new(io::ErrorKind::InvalidData, s));

    let mut grid: Grid = vec![];
    let mut num_bombs = 0;

    for line in input.lines() {
        let mut row = vec![];

        // Populate row.
        for c in line?.chars() {
            let has_bomb = match c {
                '#' => {
                    num_bombs += 1;
                    true
                }
                '.' => false,
                _ => {
                    return err(format!("Unexpected char in grid input: {}", c));
                }
            };
            row.push(Tile::new(has_bomb));
        }

        // Make sure it's a rectangle.
        if grid.len() > 0 && grid[0].len() != row.len() {
            let first = grid[0].len();
            let curr = row.len();
            let i = grid.len();
            return err(format!(
                "Jagged grid; grid[0] len {}, grid[{}] len {}",
                first, i, curr
            ));
        }

        grid.push(row);
    }

    // Check it's non-empty.
    if grid.len() == 0 || grid[0].len() == 0 {
        return err(format!("Empty grid."));
    }

    compute_adj_bombs(&mut grid);
    Ok(ParseResult { grid, num_bombs })
}

/// Populate the tiles with their correct `.adj_bombs` values.
fn compute_adj_bombs(grid: &mut Grid) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j].adj_bombs = adj_bombs(grid, i, j);
        }
    }
}

/// How many bombs are adjacent to this tile? Indeces must be in range.
///
/// (Does not include this tile itself, if its a bomb.)
fn adj_bombs(grid: &Grid, i: usize, j: usize) -> u32 {
    let i = i as isize;
    let j = j as isize;

    let n = grid.len() as isize;
    let m = grid[0].len() as isize;
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
                && grid[x as usize][y as usize].has_bomb
            {
                bombs += 1;
            }
        }
    }
    bombs
}
