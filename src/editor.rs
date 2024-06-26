use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::Error;

mod terminal;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    cursor: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();

        let result = self.repl();

        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.handle_event(&event)?;
        }
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up => {
                    let Location { x, y } = self.cursor;
                    let y = y.saturating_sub(1);
                    self.cursor.x = x;
                    self.cursor.y = y;
                }
                KeyCode::Down => {
                    let Size { height, .. } = Terminal::size()?;
                    let Location { x, y } = self.cursor;
                    let y = y.saturating_add(1);
                    let y = std::cmp::min(height.saturating_sub(1), y);
                    self.cursor.x = x;
                    self.cursor.y = y;
                }
                KeyCode::Right => {
                    let Size { width, .. } = Terminal::size()?;
                    let Location { x, y } = self.cursor;
                    let x = x.saturating_add(1);
                    //since crossterm coordinates starts from 0
                    let x = std::cmp::min(width.saturating_sub(1), x);
                    self.cursor.x = x;
                    self.cursor.y = y;
                }
                KeyCode::Left => {
                    let Location { x, y } = self.cursor;
                    let x = x.saturating_sub(1);
                    self.cursor.x = x;
                    self.cursor.y = y;
                }

                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Tata!\r\n")?;
        } else {
            Terminal::move_cursor(Position { x: 0, y: 0 })?;
            Self::draw_rows()?;
        }

        let Location { x, y } = self.cursor;
        Terminal::move_cursor(Position { x, y })?;
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Editor::draw_welcome_message()?;
            } else {
                Editor::draw_empty_row()?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editior --version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message)?;

        Ok(())
    }
}
