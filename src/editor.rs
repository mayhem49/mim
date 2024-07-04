use std::{
    io::Error,
    panic::{set_hook, take_hook},
};

use crossterm::event::{read, Event, KeyEvent, KeyEventKind};

mod editorcommand;
mod terminal;
mod view;

use terminal::Terminal;
use view::View;

use self::editorcommand::EditorCommand;

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
        let should_handle = match &event {
            Event::Resize(_, _) => true,
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            _ => false,
        };
        if should_handle {
            match EditorCommand::try_from(event) {
                Ok(command) => {
                    if matches!(command, EditorCommand::Quit) {
                        self.should_quit = true;
                    } else {
                        self.view.handle_command(command);
                    }
                }
                Err(_err) => {
                    //donot crash
                    //#[cfg(debug_assertions)]
                    //panic!("couldnot handle event: {err}")
                }
            }
        } else {
            //donot crash
            //panic!("event unsupported {event:?}");
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();

        let _ = Terminal::move_caret(self.view.get_caret_location());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}
