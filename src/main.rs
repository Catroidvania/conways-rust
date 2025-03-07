use conways::*;
use std::io::Result;

fn main() -> Result<()> {
    let mut game = Game::new().unwrap();
    
    game.run()?;

    Ok(())
}
