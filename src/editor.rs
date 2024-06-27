use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{
    io::Error,
    panic::{set_hook, take_hook},
};

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
    view: View,
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::clear_screen();
            let _ = Terminal::print("Tata!\r\n");
        }
    }
}
impl Editor {
    pub fn new() -> Result<Self, Error> {
        let curr_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            curr_hook(panic_info);
        }));
        Terminal::initialize()?;

        let args: Vec<String> = std::env::args().collect();
        let mut view = View::default();
        if let Some(file) = args.get(1) {
            assert!(
                std::path::Path::new(file).exists(),
                "file:{file} doesn't exist"
            );

            view.load(file.to_string());
        }

        Ok(Editor {
            should_quit: false,
            view,
            caret: Location::default(),
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => {
                    self.handle_event(event);
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    panic!("couldn't read event: {err:?}");
                }
            }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn handle_event(&mut self, event: Event) {
        #[allow(clippy::enum_glob_use)]
        use KeyCode::*;
        match event {
            Event::Resize(w_u16, h_u16) => {
                #[allow(clippy::as_conversions)]
                self.view.resize(Size {
                    width: w_u16 as usize,
                    height: h_u16 as usize,
                });
            }
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match code {
                Char('q') if modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                Up | Down | Right | Left | PageUp | PageDown | Home | End => {
                    self.update_caret_location(code);
                }

                _ => (),
            },
            _ => (),
        }
    }

    fn update_caret_location(&mut self, key: KeyCode) {
        #[allow(clippy::enum_glob_use)]
        use KeyCode::*;

        //note: repercussions of default?
        let Size { height, width } = Terminal::size().unwrap_or_default();
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
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();

        let Location { x, y } = self.caret;
        let _ = Terminal::move_caret(Position { x, y });
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}
