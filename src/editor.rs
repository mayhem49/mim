use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::Error;

mod terminal;
mod view;
use terminal::{Position, Size, Terminal};
use view::View;

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    caret: Location,
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

    fn update_caret_location(&mut self, key: KeyCode) -> Result<(), Error> {
        #[allow(clippy::enum_glob_use)]
        use KeyCode::*;

        let Size { height, width } = Terminal::size()?;
        let Location { mut x, mut y } = self.caret;

        match key {
            Up => {
                y = y.saturating_sub(1);
            }
            Down => {
                y = std::cmp::min(height.saturating_sub(1), y.saturating_add(1));
            }
            Left => {
                x = x.saturating_sub(1);
            }
            Right => {
                x = std::cmp::min(width.saturating_sub(1), x.saturating_add(1));
            }
            PageUp => {
                y = 0;
            }
            PageDown => {
                y = height.saturating_sub(1);
            }
            Home => {
                x = 0;
            }
            End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.caret = Location { x, y };
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            #[allow(clippy::enum_glob_use)]
            use KeyCode::*;
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                Up | Down | Right | Left | PageUp | PageDown | Home | End => {
                    self.update_caret_location(*code)?;
                }

                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Tata!\r\n")?;
        } else {
            Terminal::move_caret(Position { x: 0, y: 0 })?;
            View::render()?;
        }

        let Location { x, y } = self.caret;
        Terminal::move_caret(Position { x, y })?;
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}
