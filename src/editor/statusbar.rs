use super::{
    terminal::{Size, Terminal},
    view::location::Location,
    DocumentStatus,
};

pub struct StatusBar {
    document_status: DocumentStatus,
    redraw: bool,
    width: usize,
    y_position: usize,
    margin_bottom: usize,
}

impl StatusBar {
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();
        StatusBar {
            document_status: DocumentStatus::default(),
            redraw: true,
            width: size.width,
            y_position: size.height.saturating_sub(margin_bottom).saturating_sub(1),
            margin_bottom,
        }
    }
    pub fn resize(&mut self, size: Size) {
        self.width = size.width;
        self.y_position = size
            .height
            .saturating_sub(self.margin_bottom)
            .saturating_sub(1);
        self.redraw = true;
    }

    pub fn render(&mut self) {
        if !self.redraw {
            return;
        }
        //idk why as_deref works
        let filename = self
            .document_status
            .filename
            .as_deref()
            .unwrap_or("unnamed");

        let mut status_line = String::new();
        status_line.push_str(filename);

        if self.document_status.is_modified {
            status_line.push_str(" | ");
            status_line.push_str("[+]");
        }
        let Location { x, y } = self.document_status.curr_location;
        status_line.push_str(" | ");
        status_line.push_str(&format!("{}:{}", y + 1, x + 1));

        status_line.truncate(self.width);
        Terminal::print_row(self.y_position, &status_line).unwrap();
        self.redraw = false;
    }

    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status == self.document_status {
            return;
        }
        self.document_status = new_status;
        self.redraw = true;
    }
}
