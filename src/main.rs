use std::io::BufReader;

mod grid;

fn main() {
    // todo read the file the "right" way; also, accept cmdline args.
    let mut grid = grid::parse(BufReader::new(
        include_str!("../tests/io/small.in").as_bytes(),
    ))
    .unwrap();

    loop {
        // Display.
        print!("{}", grid.to_string());

        // Get input. todo

        // Update. todo

        break;
    }
}
