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
    pub population: HashMap<Cell, u32>,
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

    fn count_surrounding(&self, x: u16, y: u16) -> Cell {
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

        match self.cells[y as usize][x as usize] {
            Cell::Alive(c) => {
                if count == 2 || count == 3 {
                    return Cell::Alive(c);
                }
            },
            Cell::Dead => {
                if count == 3 {
                    let mut colors = colors.drain();

                    match colors.len() {
                        1 => {
                            let (c, _) = colors.nth(0).unwrap_or((Color::Grey, 0));
                            return Cell::Alive(c);
                        },
                        2 => {
                            let (c1, n1) = colors.nth(0).unwrap_or((Color::Grey, 0));
                            let (c2, n2) = colors.nth(1).unwrap_or((Color::Grey, 0));

                            // ???
                            if n1 < n2 {
                                return Cell::Alive(c1);
                            } else {
                                return Cell::Alive(c2);
                            }
                        },
                        3 => {
                            let rand = time::SystemTime::now().elapsed().unwrap().as_millis() % 3;
                            let (c, _) = colors.nth(rand as usize).unwrap();

                            return Cell::Alive(c);
                        },
                        _ => {}
                    }
                }
            }
        }
        Cell::Dead
    }

    pub fn update(&mut self) -> HashMap<Cell, u32> {
        let mut temp = Board::new(self.width, self.height);
        let mut population = HashMap::new();

        for x in 0..self.width {
            for y in 0..self.height {
                temp.set(x, y, self.count_surrounding(x, y));
                *population.entry(self.cells[y as usize][x as usize]).or_insert(0) += 1;
            }
        }

        self.cells = temp.cells;
        population
    }

    pub fn render(&mut self) -> Result<()> {
        
        //let mut x = 0;
        let mut y = 0;
        queue!(stdout(), MoveTo(0, 0))?;
        
        for r in self.cells.iter() {
            for c in r.iter() {
                //let n = self.count_surrounding(x, y);
                match c {
                    Cell::Alive(color) => {
                        queue!(stdout(),
                            SetForegroundColor(*color),
                            //SetForegroundColor(Color::Black),
                            SetBackgroundColor(*color),
                            Print("#")
                            //Print(n.to_string())
                        )?;
                    },
                    Cell::Dead => {
                        queue!(stdout(),
                            ResetColor,
                            SetForegroundColor(Color::DarkGrey),
                            //SetBackgroundColor(Color::Black),
                            Print(".")
                            //Print(n.to_string())
                        )?;
                    }
                }
                //x += 1;
            }
            //x = 0;
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
        let mut board = Board::new(w, h);
        let population = board.update();
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

        for (cell, count) in self.population.iter() {
            match cell {
                Cell::Dead => {
                    queue!(stdout(),
                        SetBackgroundColor(Color::Black),
                        SetForegroundColor(Color::White),
                        Print(" Dead: "),
                        Print(count.to_string()),
                        ResetColor)?;
                },
                Cell::Alive(c) => {
                    queue!(stdout(), 
                        Print(" "),
                        SetBackgroundColor(*c),
                        SetForegroundColor(Color::Black),
                        Print(count.to_string()),
                        ResetColor)?;
                }            
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        execute!(stdout(), EnterAlternateScreen, Hide, EnableMouseCapture)?;
        enable_raw_mode()?;

        let zero = time::Duration::from_millis(0);
        let mut last_update = time::Instant::now();

        'main: loop {
            
            self.render()?;
            stdout().flush()?;

            if !self.pause && last_update.elapsed() >= self.speed {
                last_update = time::Instant::now();
                self.population = self.board.update();
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

                self.render()?;
                stdout().flush()?;
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
