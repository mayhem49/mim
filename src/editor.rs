use std::{
    io::Error,
    panic::{set_hook, take_hook},
};

use crossterm::event::{read, Event, KeyEvent, KeyEventKind};

mod editorcommand;
mod statusbar;
mod terminal;
mod view;

use statusbar::StatusBar;
use terminal::Terminal;
use view::View;

use self::editorcommand::EditorCommand;
#[derive(Default, Debug, PartialEq, Eq)]
pub struct DocumentStatus {
    curr_location: view::location::Location,
    filename: Option<String>,
    is_modified: bool,
}

pub struct Editor {
    should_quit: bool,
    view: View,
    statusbar: StatusBar,
    title: String,
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

        let mut editor = Self {
            should_quit: false,
            statusbar: StatusBar::new(1),
            view: View::new(2),
            title: String::from("mim"),
        };

        let args: Vec<String> = std::env::args().collect();
        if let Some(file) = args.get(1) {
            editor.view.load(file);
        }

        editor.update_status();
        Ok(editor)
    }

    fn update_status(&mut self) {
        let _ = Terminal::set_title(&self.title);
        let status = self.view.get_status();
        self.statusbar.update_status(status);
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
            self.update_status();
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
                        if let EditorCommand::Resize(size) = command {
                            self.statusbar.resize(size);
                        }
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
        self.statusbar.render();
        let _ = Terminal::set_title("mim");

        let _ = Terminal::move_caret(self.view.get_caret_location());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}
