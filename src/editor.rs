use std::{
    io::Error,
    panic::{set_hook, take_hook},
};

use crossterm::event::{read, Event};

mod command;
mod commandbar;
mod messagebar;
mod statusbar;
mod terminal;
mod uicomponent;
mod view;

use command::{Action, Command, Edit};
use commandbar::CommandBar;
use messagebar::MessageBar;
use statusbar::StatusBar;
use terminal::{Position, Size, Terminal};
use uicomponent::UIComponent;
use view::View;

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
    command_bar: Option<CommandBar>,
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
            if editor.view.load(file).is_err() {
                editor
                    .messagebar
                    .update_message("couldnot load file {file}");
            };
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

    fn process_command(&mut self, command: Command) {
        //if command bar exists, forward corresspongin event to commandbar

        match command {
            Command::Edit(command) => {
                if let Some(command_bar) = self.command_bar.as_mut() {
                    if matches!(command, Edit::InsertNewLine) {
                        let filename = command_bar.get_input();
                        self.dismiss_prompt();
                        self.save_file(Some(filename));
                    } else {
                        command_bar.handle_edit_command(command);
                    }
                } else {
                    self.view.handle_edit_command(command);
                }
            }
            Command::Move(command) => {
                if self.command_bar.is_none() {
                    self.view.handle_move_command(command);
                }
            }
            Command::Action(command) => self.handle_action_command(command),
        }
    }

    fn handle_action_command(&mut self, action: command::Action) {
        match action {
            Action::Save => self.handle_save(),
            Action::Quit => self.handle_quit(),
            Action::ForceQuit => self.handle_force_quit(),
            Action::Resize(size) => self.resize(size),
            Action::Dismiss => {
                if self.command_bar.is_some() {
                    self.dismiss_prompt();
                    self.messagebar.update_message("Save Aborted");
                }
            }
        }
    }
    fn save_file(&mut self, filename: Option<String>) {
        let result = if let Some(filename) = filename {
            self.view.save_as(filename)
        } else {
            self.view.save()
        };
        if result.is_ok() {
            self.messagebar.update_message("File saved successfully");
        } else {
            self.messagebar.update_message("Errow while saving file");
        }
    }
    fn handle_save(&mut self) {
        if self.view.is_unnamed() {
            self.show_prompt("Save As:");
        } else {
            self.save_file(None);
        }
    }

    fn handle_force_quit(&mut self) {
        self.should_quit = true;
    }
    fn handle_quit(&mut self) {
        if self.view.is_modified() {
            self.messagebar
                .update_message("please save the file before closing");
        } else {
            self.should_quit = true;
        }
    }

    fn handle_event(&mut self, event: Event) {
        if let Ok(command) = Command::try_from(event) {
            self.process_command(command);
        }
    }
    fn show_prompt(&mut self, prompt: &str) {
        let mut command_bar = CommandBar::default();
        command_bar.resize(self.size);
        command_bar.update_prompt(prompt);
        command_bar.mark_redraw(true);
        self.command_bar = Some(command_bar);
    }
    fn dismiss_prompt(&mut self) {
        self.command_bar = None;
        self.messagebar.mark_redraw(true);
    }
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        let Size { width, height } = self.size;
        self.view.resize(Size {
            height: height.saturating_sub(2),
            width,
        });
        self.statusbar.resize(Size { height: 1, width });
        if let Some(command_bar) = self.command_bar.as_mut() {
            command_bar.resize(Size { height: 1, width });
        } else {
            self.messagebar.resize(Size { height: 1, width });
        }
    }

    fn refresh_screen(&mut self) {
        if self.size.height == 0 || self.size.width == 0 {
            return;
        }

        self.render_title();
        let _ = Terminal::hide_caret();

        let bottom_row = self.size.height.saturating_sub(1);
        if let Some(command_bar) = self.command_bar.as_mut() {
            command_bar.render(bottom_row);
        } else {
            self.messagebar.render(bottom_row);
        }

        if self.size.height > 1 {
            self.statusbar.render(self.size.height.saturating_sub(2));
        }
        if self.size.height > 2 {
            self.view.render(0);
        }
        //handle title too

        let caret_position = if let Some(command_bar) = self.command_bar.as_ref() {
            Position {
                row: bottom_row,
                col: command_bar.get_caret_location(),
            }
        } else {
            self.view.get_caret_location()
        };
        let _ = Terminal::move_caret(caret_position);
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}
