use conways::*;
use std::{
    io::Result,
    env::args,
};

fn main() -> Result<()> {
    let mut game = Game::new(Config::new(args())).unwrap();

    game.run()?;

    Ok(())
}
