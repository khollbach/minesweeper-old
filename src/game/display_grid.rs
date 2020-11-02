use super::Grid;
use std::fmt;

impl fmt::Display for Grid {
    /// Rows of the grid, with no trailing newline.
    ///
    /// See [`Tile::to_char`] for more details.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();

        for (i, row) in self.iter().enumerate() {
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
