use crossterm::{
    cursor::{
        MoveTo
    },
    event::read,
    execute,
    queue,
    style::{
        Color,
        ResetColor,
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

#[derive(Copy, Clone)]
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
    pub board: Board,
    pub speed: u32,
    pub pause: bool
}

impl Board {
    pub fn new(width: u16, height: u16) -> Board {
        Board {
            width,
            height,
            cells: vec![vec![Cell::Dead; width.into()]; height.into()]
        }
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y as usize][x as usize] = cell;
        }
    }

    pub fn render(&mut self) -> Result<()> {
        
        let mut y = 0;
        queue!(stdout(), MoveTo(0, 0))?;
        
        for r in self.cells.iter() {
            for c in r.iter() {
                match c {
                    Cell::Alive(color) => {
                        queue!(stdout(),
                            SetForegroundColor(*color),
                            SetBackgroundColor(*color),
                            Print("#")
                        )?;
                    },
                    Cell::Dead => {
                        queue!(stdout(),
                            SetForegroundColor(Color::DarkGrey),
                            SetBackgroundColor(Color::Black),
                            Print(".")
                        )?;
                    }
                }
            }
            y += 1;
            queue!(stdout(), ResetColor, MoveTo(0, y))?;
        }

        stdout().flush()?;
        Ok(())
    }
}

impl Game {
    pub fn new() -> Result<Game> {
        let (w, h) = size()?;
        Ok(Game {
            board: Board::new(w, h),
            speed: 1000,
            pause: true
        })
    }

    pub fn run(&mut self) -> Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;

        self.board.set(0, 0, Cell::Alive(Color::Grey));
        self.board.set(2, 0, Cell::Alive(Color::Red));
        self.board.set(0, 2, Cell::Alive(Color::Blue));

        self.board.render()?;

        stdout().flush()?;

        read()?;

        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}
