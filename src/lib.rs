use crossterm::{
    cursor::{
        MoveTo
    },
    event::read,
    execute,
    queue,
    style::{
        Color,
        SetBackgroundColor,
        SetForegroundColor,
        Print
    },
    terminal::{
        EnterAlternateScreen,
        LeaveAlternateScreen,
        enable_raw_mode,
        disable_raw_mode,
        size
        },
    };

use std::io::{stdout, Result, Write};

#[derive(Clone)]
pub enum Cell {
    Alive(Color),
    Dead
}

pub struct Board {
    pub width: u16,
    pub height: u16,
    cells: Vec<Vec<Cell>>
}

pub struct Game {
    pub cells: Board,
    pub speed: u32,
    pub pause: bool
}

impl Cell {

}

impl Board {
    pub fn new(width: u16, height: u16) -> Board {
        Board {
            width,
            height,
            cells: vec![vec![Cell::Dead; height.into()]; width.into()]
        }
    }
}

impl Game {
    pub fn new() -> Result<Game> {
        let (w, h) = size()?;
        Ok(Game {
            cells: Board::new(w, h),
            speed: 1000,
            pause: true
        })
    }

    pub fn run(&mut self) -> Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;

        queue!(stdout(),
            MoveTo(10, 10),
            SetForegroundColor(Color::Red),
            Print("#"),
            SetForegroundColor(Color::Blue),
            Print("#")
            )?;

        stdout().flush()?;

        read()?;

        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}
