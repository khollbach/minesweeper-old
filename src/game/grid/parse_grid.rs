use super::{Grid, Tile};
use std::io::{self, BufRead, BufReader, Read};

/// Return value of [`parse_grid`].
pub struct ParseResult {
    pub grid: Grid,
    pub num_bombs: u32,
}

/// Parse an input stream into a Grid. The grid must be non-empty and rectangular.
///
/// Valid input chars are 'x' or '.' for bomb / empty. Otherwise, this will fail.
pub fn parse_grid(input: &mut BufReader<impl Read>) -> io::Result<ParseResult> {
    let err = |s| Err(io::Error::new(io::ErrorKind::InvalidData, s));

    let mut grid = Grid::new(vec![]);
    let mut num_bombs = 0;

    for line in input.lines() {
        let mut row = vec![];

        // Populate row.
        for c in line?.chars() {
            let has_bomb = match c {
                'x' => {
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
/// We do include this tile itself, if it is a bomb.
fn adj_bombs(grid: &Grid, i: usize, j: usize) -> u32 {
    let i = i as isize;
    let j = j as isize;

    let n = grid.len() as isize;
    let m = grid[0].len() as isize;
    assert!((0..n).contains(&i) && (0..m).contains(&j));

    let mut bombs = 0;

    // Check all 9 "adjacent" tiles for bombs.
    for &di in &[-1, 0, 1] {
        for &dj in &[-1, 0, 1] {
            let x = i + di;
            let y = j + dj;
            if (0..n).contains(&x) && (0..m).contains(&y) && grid[x as usize][y as usize].is_bomb {
                bombs += 1;
            }
        }
    }

    assert!(bombs <= 9);
    bombs
}

#[cfg(test)]
mod tests {
    use super::super::Visibility;
    use super::*;
    use std::fs::{self, File};
    use std::io::BufReader;

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

    /// Parse the input file, and then print it back to a string.
    ///
    /// Check that it matches the output file.
    fn good(test_name: &'static str) {
        let repo_root = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/tests/parse_grid/good/{}", repo_root, test_name);

        // Parse the input file.
        let mut input = BufReader::new(File::open(&format!("{}.in", path)).unwrap());
        let ParseResult {
            mut grid,
            num_bombs,
        } = parse_grid(&mut input).unwrap();

        assert_eq!(num_bombs, count_bombs(&grid));

        // Display the grid.
        reveal_all(&mut grid);
        let mut output = grid.to_string();
        output.push('\n');

        let expected = fs::read_to_string(format!("{}.out", path)).unwrap();
        assert_eq!(output, expected);
    }

    fn count_bombs(grid: &Grid) -> u32 {
        let mut num_bombs = 0;
        for row in grid.iter() {
            for tile in row {
                if tile.is_bomb {
                    num_bombs += 1;
                }
            }
        }
        num_bombs
    }

    fn reveal_all(grid: &mut Grid) {
        for row in grid.iter_mut() {
            for tile in row {
                tile.visibility = Visibility::Revealed;
            }
        }
    }

    /// Try to parse a malformed game grid, and unwrap an Err.
    fn bad(test_name: &'static str) {
        let repo_root = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/tests/parse_grid/bad/{}", repo_root, test_name);

        // Parse the input file.
        let mut input = BufReader::new(File::open(&format!("{}", path)).unwrap());
        let result = parse_grid(&mut input);
        assert!(result.is_err());
    }
}
