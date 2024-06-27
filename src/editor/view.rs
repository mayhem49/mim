use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};

mod buffer;
mod line;
mod location;

use buffer::Buffer;
use location::Location;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}

impl Default for View {
    fn default() -> Self {
        View {
            buffer: Buffer::default(),
            size: Terminal::size().unwrap_or_default(),
            redraw: true,
            location: Location::default(),
            scroll_offset: Location::default(),
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
        let Location {
            x: scroll_x,
            y: scroll_y,
        } = self.scroll_offset;

        for current_row in 0..height {
            #[allow(clippy::integer_division)]
            let vertical_center = height / 3;

            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(scroll_y)) {
                //not utf compliant?
                let left = scroll_x;
                let right = scroll_x.saturating_add(width);
                Self::render_line(current_row, &line.get(left..right));
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

    fn move_text_location(&mut self, direction: &Direction) {
        let Size { height, width } = self.size;
        let Location { mut x, mut y } = self.location;

        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            }
            Direction::Down => {
                y = y.saturating_add(1);
                //y = std::cmp::min(height.saturating_sub(1), y.saturating_add(1)); y = std::cmp::min(height.saturating_sub(1), y.saturating_add(1));
            }
            Direction::Left => {
                x = x.saturating_sub(1);
            }
            Direction::Right => {
                x = x.saturating_add(1);
                //x = std::cmp::min(width.saturating_sub(1), x.saturating_add(1));
            }
            Direction::PageUp => {
                y = 0;
            }
            Direction::PageDown => {
                y = height.saturating_sub(1);
            }
            Direction::Home => {
                x = 0;
            }
            Direction::End => {
                x = width.saturating_sub(1);
            }
        }
        self.location = Location { x, y };
        self.update_scroll_offset();
    }

    fn update_scroll_offset(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;

        self.scroll_offset = Location {
            x: x.saturating_add(1).saturating_sub(width),
            y: y.saturating_add(1).saturating_sub(height),
        };
        //todo: redraw only if scroll offset changes
        self.redraw();
    }

    pub fn get_caret_location(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {}
        }
    }
}
