use std::{
    io::Error,
    panic::{set_hook, take_hook},
};

use crossterm::event::{read, Event, KeyEvent, KeyEventKind};

mod editorcommand;
mod messagebar;
mod statusbar;
mod terminal;
mod uicomponent;
mod view;

use messagebar::MessageBar;
use statusbar::StatusBar;
use terminal::{Size, Terminal};
use uicomponent::UIComponent;
use view::View;

use self::editorcommand::EditorCommand;
#[derive(Default, Debug, PartialEq, Eq)]
pub struct DocumentStatus {
    curr_location: view::location::Location,
    filename: Option<String>,
    is_modified: bool,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    statusbar: StatusBar,
    messagebar: MessageBar,
    title: String,
    size: Size,
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

        let mut editor = Editor::default();
        let size = Terminal::size().unwrap_or_default();
        editor.resize(size);

        let args: Vec<String> = std::env::args().collect();
        if let Some(file) = args.get(1) {
            editor.view.load(file);
        }

        editor.update_status();
        Ok(editor)
    }

    fn update_status(&mut self) {
        let status = self.view.get_status();
        self.statusbar.update_status(status);
    }

    fn render_title(&mut self) {
        let _ = Terminal::set_title(&self.title);
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
                    } else if let EditorCommand::Resize(size) = command {
                        self.resize(size);
                    } else {
                        self.view.handle_command(command);
                    }
                }
                Err(_err) => {}
            }
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
        let Size { width, height } = self.size;
        self.view.resize(Size {
            height: height.saturating_sub(2),
            width,
        });
        self.statusbar.resize(Size { height: 1, width });
        self.messagebar.resize(Size { height: 1, width });
    }

    fn refresh_screen(&mut self) {
        if self.size.height == 0 || self.size.width == 0 {
            return;
        }

        self.render_title();
        let _ = Terminal::hide_caret();

        self.messagebar.render(self.size.height.saturating_sub(1));
        if self.size.height > 1 {
            self.statusbar.render(self.size.height.saturating_sub(2));
        }
        if self.size.height > 2 {
            self.view.render(0);
        }
        //handle title too

        let _ = Terminal::move_caret(self.view.get_caret_location());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}
