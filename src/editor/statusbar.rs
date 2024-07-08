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
        let right_width = self.width / 2;
        let left_width = self.width.saturating_sub(right_width); // to handle odd width
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

        Terminal::print_inverted_row(self.y_position, &status_line).unwrap();
        self.redraw = false;
    }

    pub fn update_status(&mut self, new_status: DocumentStatus) {
        use log::info;
        info!("{:?}", new_status);
        if new_status == self.document_status {
            return;
        }
        self.document_status = new_status;
        self.redraw = true;
    }
}
