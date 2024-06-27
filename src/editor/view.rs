use super::{
    terminal::{Size, Terminal},
    Location,
};

mod buffer;
use buffer::Buffer;
use crossterm::event::{KeyCode, KeyEvent};
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    redraw: bool,
    size: Size,
    caret: Location,
}

impl Default for View {
    fn default() -> Self {
        View {
            buffer: Buffer::default(),
            size: Terminal::size().unwrap_or_default(),
            redraw: true,
            caret: Location::default(),
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
    fn render_line(at: usize, line: &str) {
        let result = Terminal::print_row(at, line);
        debug_assert!(result.is_ok(), "Failed to render line");
    }
    pub fn render(&mut self) {
        if !self.redraw {
            return;
        }
        self.redraw = false;
        let Size { height, width } = self.size;

        for current_row in 0..height {
            #[allow(clippy::integer_division)]
            let vertical_center = height / 3;

            if let Some(line) = self.buffer.lines.get(current_row) {
                //not utf compliant?
                let render_len = std::cmp::min(line.len(), width);
                Self::render_line(current_row, line.get(0..render_len).unwrap());
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
    }

    pub fn load(&mut self, file: &str) {
        if let Ok(buffer) = Buffer::load(file) {
            self.buffer = buffer;
        }
        self.redraw = true;
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.redraw();
    }

    pub fn redraw(&mut self) {
        self.redraw = true;
    }

    pub fn handle_key_press(&mut self, key_event: KeyEvent) {
        #[allow(clippy::enum_glob_use)]
        use KeyCode::*;

        let KeyEvent { code, .. } = key_event;
        match code {
            Up | Down | Right | Left | PageUp | PageDown | Home | End => {
                self.update_caret_location(code);
            }

            _ => (),
        }
    }

    fn update_caret_location(&mut self, key: KeyCode) {
        #[allow(clippy::enum_glob_use)]
        use KeyCode::*;

        //note: repercussions of default?
        let Size { height, width } = Terminal::size().unwrap_or_default();
        let Location { mut x, mut y } = self.caret;

        match key {
            Up => {
                y = y.saturating_sub(1);
            }
            Down => {
                y = std::cmp::min(height.saturating_sub(1), y.saturating_add(1));
            }
            Left => {
                x = x.saturating_sub(1);
            }
            Right => {
                x = std::cmp::min(width.saturating_sub(1), x.saturating_add(1));
            }
            PageUp => {
                y = 0;
            }
            PageDown => {
                y = height.saturating_sub(1);
            }
            Home => {
                x = 0;
            }
            End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.caret = Location { x, y };
    }

    pub fn get_caret_location(&mut self) -> Location {
        self.caret
    }
}
