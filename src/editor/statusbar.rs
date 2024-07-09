use super::{
    terminal::{Size, Terminal},
    uicomponent::UIComponent,
    view::location::Location,
    DocumentStatus,
};
use std::io::Error;

#[derive(Default)]
pub struct StatusBar {
    document_status: DocumentStatus,
    redraw: bool,
    size: Size,
}

impl StatusBar {
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status == self.document_status {
            return;
        }
        self.document_status = new_status;
        self.mark_redraw(true);
    }
}

impl UIComponent for StatusBar {
    fn mark_redraw(&mut self, redraw: bool) {
        self.redraw = redraw;
    }
    fn needs_redraw(&self) -> bool {
        self.redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn draw(&self, start_y: usize) -> Result<(), Error> {
        let filename = self
            .document_status
            .filename
            .as_deref()
            .unwrap_or("unnamed");

        let Location { x, y } = self.document_status.curr_location;

        #[allow(clippy::arithmetic_side_effects)]
        let right_section = format!("{}:{}", y + 1, x + 1);

        let left_section = if self.document_status.is_modified {
            format!("{} | {}", filename, "[+]")
        } else {
            filename.to_string()
        };

        #[allow(clippy::integer_division)]
        let right_width = self.size.width / 2;
        let left_width = self.size.width.saturating_sub(right_width); // to handle odd width
        let pad = 2;
        let left_pad = pad;
        let right_pad = pad;
        let left_inner_width = left_width.saturating_sub(left_pad);
        let right_inner_width = right_width.saturating_sub(right_pad);

        // <left-pad><left-section><right-section><right-pad>
        let status_line = format!(
            "{empty:left_pad$}{:<left_inner_width$}{:>right_inner_width$}{empty:>right_pad$}",
            left_section,
            right_section,
            empty = ""
        );

        Terminal::print_inverted_row(start_y, &status_line).unwrap();
        Ok(())
    }
}
