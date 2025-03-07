use conways::*;
use std::io::Result;

fn main() -> Result<()> {
    let mut game = Game::new().unwrap();

    // println!("{}:{}", game.board.width, game.board.height);
    game.run()?;

    Ok(())
}
