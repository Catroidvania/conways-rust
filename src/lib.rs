use crossterm::{
    cursor::{
        MoveTo,
        Hide,
        Show,
    },
    event::*/*{
        read,
        poll,
        Event,
        KeyCode,
        MouseEventKind,
        MouseButton,
    }*/,
    execute,
    queue,
    style::*/*{
        Color,
        ResetColor,
        SetBackgroundColor,
        SetForegroundColor,
        Print,
    }*/,
    terminal::*/*{
        EnterAlternateScreen,
        LeaveAlternateScreen,
        enable_raw_mode,
        disable_raw_mode,
        size,
        }*/,
    };
use std::{
    io::{
        stdout,
        Result,
        Write
    },
    time::Duration,
    };


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

        Ok(())
    }

    pub fn clear(&mut self) {
        for r in self.cells.iter_mut() {
            for c in r.iter_mut() {
                *c = Cell::Dead;
            }
        }
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

    pub fn render(&mut self) -> Result<()> {

        self.board.render()?;
        queue!(stdout(), MoveTo(0, self.board.height-1), Print("Speed: "))?;

        if self.pause {
            queue!(stdout(), Print("PAUSE"))?;
        } else {
            queue!(stdout(), Print(self.speed.to_string()))?;
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        execute!(stdout(), EnterAlternateScreen, Hide, EnableMouseCapture)?;
        enable_raw_mode()?;

        /*
        self.board.set(0, 0, Cell::Alive(Color::Grey));
        self.board.set(2, 0, Cell::Alive(Color::Red));
        self.board.set(0, 2, Cell::Alive(Color::Blue));
        */
        self.render()?;
        stdout().flush()?;

        'main: loop {

            //self.board.clear();

            while let Some(event) = Self::wait_event(20) {
                match event {
                    Event::Key(k_event) => {
                        if let KeyCode::Char(c) = k_event.code {
                            match c {
                                'q' => {
                                    break 'main;
                                },
                                'p' => {
                                    self.pause = !self.pause;
                                },
                                _ => {},
                            }
                        }
                    },
                    Event::Mouse(m_event) => {
                        match m_event.kind {
                            MouseEventKind::Down(btn) | MouseEventKind::Drag(btn) => {
                                match btn {
                                    MouseButton::Left => {
                                        self.board.set(m_event.column, m_event.row, Cell::Alive(Color::Blue));
                                    },
                                    MouseButton::Right => {
                                        self.board.set(m_event.column, m_event.row, Cell::Dead);
                                    },
                                    _ => {},
                                }
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }

            self.render()?;
            stdout().flush()?;
        }

        execute!(stdout(), LeaveAlternateScreen, Show, DisableMouseCapture)?;
        disable_raw_mode()?;

        Ok(())
    }

    fn wait_event(delay: u64) -> Option<Event> {
        if poll(Duration::from_millis(delay)).unwrap_or(false) {
            if let Ok(event) = read() {
                return Some(event);
            }
        }
        None
    }
}
