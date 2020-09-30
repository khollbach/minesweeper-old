use std::error;

mod game;

use game::Game;

fn main() -> Result<(), Box<dyn error::Error>> {
    // todo accept cmdline args.

    let mut game = Game::from_file("tests/io/good/small.in")?;

    loop {
        // Display.
        println!("{}\n", game.to_string());
        game.reveal_all();
        println!("{}", game.to_string());

        // Get input. todo

        // Update. todo

        break;
    }

    Ok(())
}
