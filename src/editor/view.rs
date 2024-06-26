use super::terminal::{Position, Size, Terminal};
use std::io::Error;

mod buffer;
use buffer::Buffer;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        View {
            buffer: Buffer::default(),
            size: Terminal::size().unwrap_or_default(),
            redraw: true,
        }
    }
}
impl View {
    fn build_welcome_message(width: usize) -> String {
        let mut welcome_message = format!("{NAME} editior --version {VERSION}");
        let len = welcome_message.len();

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;

        welcome_message = format!("~{}{welcome_message}", " ".repeat(padding));
        welcome_message.truncate(width);
        welcome_message
    }
    fn draw_line(at: usize, line: &str) -> Result<(), Error> {
        Terminal::move_caret(Position { x: 0, y: at })?;

        Terminal::clear_line()?;
        Terminal::print(line)?;
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), Error> {
        if !self.redraw {
            return Ok(());
        }
        self.redraw = false;
        let Size { height, width } = Terminal::size()?;

        for current_row in 0..height {
            #[allow(clippy::integer_division)]
            let vertical_center = height / 3;

            if let Some(line) = self.buffer.lines.get(current_row) {
                //not utf compliant?
                let render_len = std::cmp::min(line.len(), width);
                Self::draw_line(current_row, line.get(0..render_len).unwrap())?;
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::draw_line(current_row, &Self::build_welcome_message(width))?;
            } else {
                Self::draw_line(current_row, "~")?;
            }
        }
        Ok(())
    }

    pub fn load(&mut self, file: String) {
        if let Ok(buffer) = Buffer::load(file) {
            self.buffer = buffer;
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.redraw();
    }

    pub fn redraw(&mut self) {
        self.redraw = true;
    }
}
