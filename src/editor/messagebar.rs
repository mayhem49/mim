use super::{
    terminal::{Size, Terminal},
    uicomponent::UIComponent,
};
use std::io::Error;

#[derive(Default)]
pub struct MessageBar {
    message: Option<String>,
    redraw: bool,
    size: Size,
}

impl MessageBar {
    pub fn update_message(&mut self, msg: &str) {
        self.message = Some(msg.to_string());
    }
}

impl UIComponent for MessageBar {
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
        let msg = self.message.as_deref().unwrap_or_default();
        Terminal::print_row(y_position, msg)?;
        Ok(())
    }
}
