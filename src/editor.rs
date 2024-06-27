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

            view.load(file);
        }

        Ok(Editor {
            should_quit: false,
            view,
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
        match event {
            Event::Resize(w_u16, h_u16) => {
                #[allow(clippy::as_conversions)]
                self.view.resize(Size {
                    width: w_u16 as usize,
                    height: h_u16 as usize,
                });
            }
            Event::Key(
                key_event @ KeyEvent {
                    code, modifiers, ..
                },
            ) => match code {
                KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => self.view.handle_key_press(key_event),
            },
            _ => (),
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();

        let Location { x, y } = self.view.get_caret_location();
        let _ = Terminal::move_caret(Position { x, y });
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}
