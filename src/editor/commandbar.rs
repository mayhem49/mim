use super::{
    command::Edit,
    terminal::{Size, Terminal},
    uicomponent::UIComponent,
    view::line::Line,
};
use std::io::Error;

#[derive(Default)]
pub struct CommandBar {
    prompt: String,
    input: Line,
    redraw: bool,
    size: Size,
}

impl CommandBar {
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(char) => self.input.append_char(char),
            Edit::DeleteBackward => self.input.remove_last(),
            Edit::Delete | Edit::InsertNewLine => {}
        }
        self.mark_redraw(true);
    }
    pub fn update_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
        self.mark_redraw(true);
    }

    pub fn get_caret_location(&self) -> usize {
        let total_width = self
            .prompt
            .len()
            .saturating_add(self.input.grapheme_count());
        std::cmp::min(total_width, self.size.width)
    }

    pub fn get_input(&self) -> String {
        self.input.to_string()
    }
}
//It should replace the Message Bar on press of ctrl+s if the currently open file has no file name.
//Hitting Esc should dismiss the prompt without saving. We should display Save aborted.
//Hitting Enter if data was entered attempts to save the current file to the given file name.
//Dismissing the Command Bar should show the message bar again.
impl UIComponent for CommandBar {
    fn mark_redraw(&mut self, redraw: bool) {
        self.redraw = redraw;
    }

    fn needs_redraw(&self) -> bool {
        self.redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&self, y_position: usize) -> Result<(), Error> {
        let input_render_len = self.size.width.saturating_sub(self.prompt.len());
        let right = self.input.width();
        let left = right.saturating_sub(input_render_len);
        let truncated_input = self.input.get_graphemes(left..right);
        let command_line = format!("{}{}", self.prompt, truncated_input);
        Terminal::print_row(y_position, &command_line)?;
        Ok(())
    }
}
