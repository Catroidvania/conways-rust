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
    collections::HashMap,
    io::{
        stdout,
        Result,
        Write
    },
    time,
    thread,
    };
use itertools::Itertools;
use rand;


#[derive(Copy, Clone, Eq, Hash, PartialEq)]
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
    pub speed: time::Duration,
    pub pause: bool,
    pub color: Color,
    pub population: HashMap<Color, u64>,
}


impl Board {
    pub fn new(width: u16, height: u16) -> Board {
        Board {
            width,
            height,
            cells: vec![vec![Cell::Dead; width as usize]; height as usize]
        }
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y as usize][x as usize] = cell;
        }
    }

    fn update_cell(&self, x: u16, y: u16) -> (Cell, u8) {
        let mut colors = HashMap::new();
        let mut count = 0;
        
        for xo in -1..=1 {
            for yo in -1..=1 { 
                if xo == 0 && yo == 0 {
                    continue;
                }

                let nx = x as i32 + xo;
                let ny = y as i32 + yo;
                if nx < 0 || nx >= self.width as i32 || ny < 0 || ny >= self.height as i32 {
                    continue;
                }

                if let Cell::Alive(c) = self.cells[ny as usize][nx as usize] {
                    *colors.entry(c).or_insert(0) += 1;
                    count += 1;
                }
            }
        }

        let mut new_cell = Cell::Dead;
        let mut len: u8 = 0;
        match self.cells[y as usize][x as usize] {
            Cell::Alive(c) => {
                if count == 2 || count == 3 {
                    new_cell = Cell::Alive(c);
                }
            },
            Cell::Dead => {
                if count == 3 {
                    len = colors.keys().len() as u8;
                    let colors: Vec<(Color, u8)> = colors.drain().collect();

                    match len {
                        1 => {
                            let (c, _) = colors[0];
                            new_cell = Cell::Alive(c);
                        },
                        2 => {
                            let (c1, n1) = colors[0];
                            let (c2, n2) = colors[1];

                            new_cell = Cell::Alive(if n1 > n2 { c1 } else { c2 });
                        },
                        3 => {
                            let rn: u8 = rand::random();
                            let (c, _) = colors[(rn % 3) as usize];
                            new_cell = Cell::Alive(c);
                        },
                        _ => {}
                    }
                }
            }
        }
        (new_cell, len)
    }

    fn count_population(&self) -> HashMap<Color, u64> {
        let mut population = HashMap::new();

        for x in 0..self.width {
            for y in 0..self.height {
                if let Cell::Alive(c) = self.cells[y as usize][x as usize] {
                    *population.entry(c).or_insert(0) += 1;
                }
            }
        }

        population
    }

    pub fn update(&mut self) {
        let mut temp = Board::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                temp.set(x, y, self.update_cell(x, y).0);
            }
        }

        self.cells = temp.cells;
    }

    pub fn render(&mut self) -> Result<()> {
        
        let mut x = 0;
        let mut y = 0;
        queue!(stdout(), MoveTo(0, 0))?;
        
        for r in self.cells.iter() {
            for c in r.iter() {
                let (cell, len) = self.update_cell(x, y);
                let test_c = match cell {
                    Cell::Alive(col) => col,
                    Cell::Dead => Color::DarkGrey
                };
                match c {
                    Cell::Alive(color) => {
                        queue!(stdout(),
                            ResetColor,
                            //SetForegroundColor(*color),
                            SetForegroundColor(test_c),
                            SetBackgroundColor(*color),
                            Print(len.to_string())
                        )?;
                    },
                    Cell::Dead => {
                        queue!(stdout(),
                            ResetColor,
                            //SetForegroundColor(Color::DarkGrey),
                            SetForegroundColor(test_c),
                            Print(len.to_string())
                        )?;
                    }
                }
                x += 1;
            }
            x = 0;
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
        let board = Board::new(w, h);
        let population = board.count_population();
        Ok(Game {
            board,
            speed: time::Duration::from_millis(200),
            pause: true,
            color: Color::White,
            population,
        })
    }

    pub fn render(&mut self) -> Result<()> {
        self.board.render()?;

        queue!(stdout(), 
            MoveTo(0, self.board.height-1),
            SetBackgroundColor(self.color),
            SetForegroundColor(self.color),
            Print("#"),
            ResetColor,
            Print(" Speed: "))?;

        if self.pause {
            queue!(stdout(), Print("PAUSE"))?;
        } else {
            queue!(stdout(), Print(self.speed.as_millis().to_string()))?;
        }

        for (color, count) in self.population.iter().sorted() {
            let percent = format!("{:.4}%", *count as f64 / (self.board.width * self.board.height) as f64);
            queue!(stdout(), 
                Print(" "),
                SetBackgroundColor(*color),
                SetForegroundColor(
                    match color {
                        Color::DarkGrey | Color::Grey => Color::White,
                        _ => Color::Black
                    }
                ),
                Print(percent),
                ResetColor)?;    
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        execute!(stdout(), EnterAlternateScreen, Hide, EnableMouseCapture)?;
        enable_raw_mode()?;

        let zero = time::Duration::from_millis(0);
        let mut last_update = time::Instant::now();

        'main: loop {
            
            self.population = self.board.count_population();
            self.render()?;
            stdout().flush()?;

            if !self.pause && last_update.elapsed() >= self.speed {
                last_update = time::Instant::now();
                self.board.update();
            }

            while let Some(event) = Self::wait_event(zero) {
                match event {
                    Event::Mouse(m_event) => {
                        match m_event.kind {
                            MouseEventKind::Down(btn) | MouseEventKind::Drag(btn) => {
                                match btn {
                                    MouseButton::Left =>    self.board.set(m_event.column, m_event.row, Cell::Alive(self.color)),
                                    MouseButton::Right =>   self.board.set(m_event.column, m_event.row, Cell::Dead),
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    },
                    Event::Key(k_event) => {
                        if k_event.kind == KeyEventKind::Press {
                            match k_event.code {
                                KeyCode::Char(c) => {
                                    match c {
                                        ' ' => {
                                            self.pause = !self.pause;
                                            last_update = time::Instant::now();
                                        },
                                        '1' => self.color = Color::White,
                                        '2' => self.color = Color::Red,
                                        '3' => self.color = Color::Yellow,
                                        '4' => self.color = Color::Green,
                                        '5' => self.color = Color::Blue,
                                        '6' => self.color = Color::Cyan,
                                        '7' => self.color = Color::Magenta,
                                        '8' => self.color = Color::DarkGrey,
                                        _ => {}
                                    }
                                },
                                KeyCode::Esc => break 'main,
                                KeyCode::Backspace | KeyCode::Delete => self.board.clear(),
                                _ => {}
                            }
                        }    
                    },
                    _ => {}
                }

                /*
                self.population = self.board.count_population();
                self.render()?;
                stdout().flush()?;
                */
            }

            thread::sleep(time::Duration::from_millis(15));
        }

        execute!(stdout(), LeaveAlternateScreen, Show, DisableMouseCapture)?;
        disable_raw_mode()?;

        Ok(())
    }

    fn wait_event(delay: time::Duration) -> Option<Event> {
        if poll(delay).unwrap_or(false) {
            if let Ok(event) = read() {
                return Some(event);
            }
        }
        None
    }
}
