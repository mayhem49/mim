//one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty twenty-one twenty-two twenty-three twenty-four twenty-five twenty-six twenty-seven twenty-eight twenty-nine thirty thirty-one thirty-two thirty-three thirty-four thirty-five thirty-six thirty-seven thirty-eight thirty-nine forty forty-one forty-two forty-three forty-four forty-five forty-six forty-seven forty-eight forty-nine fifty fifty-one fifty-two fifty-three fifty-four fifty-five
use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};

mod buffer;
mod line;
mod location;

use buffer::Buffer;
use line::Line;
use location::Location;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    redraw: bool,
    size: Size,
    //Todo: rename after configuring nvim
    location: Location,
    scroll_offset: Position,
}

impl Default for View {
    fn default() -> Self {
        View {
            buffer: Buffer::default(),
            size: Terminal::size().unwrap_or_default(),
            redraw: true,
            location: Location::default(),
            scroll_offset: Position::default(),
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
        //info!("size:w,h {:?},{:?}", self.size.width, self.size.height);
        //log::info!(
        //"text_location:x,y {:?},{:?}",
        //self.location.x,
        //self.location.y
        //);
        //log::info!(
        //"scroll:x,y {:?},{:?}",
        //self.scroll_offset.col,
        //self.scroll_offset.row
        //);
        //let x = self
        //    .buffer
        //    .lines
        //    .get(self.location.y)
        //    .map_or(0, |line| line.width_until(self.location.x));
        //let xes = self
        //    .buffer
        //    .lines
        //    .get(self.location.y)
        //    .map_or(0, Line::grapheme_count);
        //log::info!("x {x} graphemes: {xes}");
        //log::info!("");

        if !self.redraw {
            return;
        }
        self.redraw = false;
        let Size { height, width } = self.size;
        let Position {
            col: scroll_x,
            row: scroll_y,
        } = self.scroll_offset;

        for current_row in 0..height {
            #[allow(clippy::integer_division)]
            let vertical_center = height / 3;

            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(scroll_y)) {
                //not utf compliant?
                let left = scroll_x;
                let right = scroll_x.saturating_add(width);
                Self::render_line(current_row, &line.get_graphemes(left..right));
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
        self.update_scroll_offset();
        self.redraw();
    }

    pub fn insert_char(&mut self, char: char) {
        //handle enter
        let Location { y, x: _ } = self.location;

        //handle None
        let old_graphemes = self.buffer.lines.get(y).map_or(0, Line::grapheme_count);

        self.buffer.insert_char(char, self.location);
        let new_graphemes = self.buffer.lines.get(y).map_or(0, Line::grapheme_count);

        if old_graphemes == new_graphemes {
        } else {
            self.move_text_location(&Direction::Right);
            self.redraw();
        }
    }

    pub fn redraw(&mut self) {
        self.redraw = true;
    }

    /// UP
    /// moves the cursor upward by 1
    /// DOWN
    /// moves the cursor downward by 1
    /// LEFT
    /// moves the cursor left by 1
    /// RIGHT
    /// moves the cursor rightward by 1
    /// maximum right is end of line
    /// HOME
    /// moves the cursor to start of line
    /// END
    /// moves the cursor to end of the line
    ///
    /// PAGEUP
    /// scroll the page upward by current height of the terminal
    /// cursor will be at the bottom of the view
    /// PAGEDOWN
    /// scroll the page downward by current height of the terminal
    ///
    /// during vertical movement: x will be adjusted if x of prev line is greater then line length
    fn move_text_location(&mut self, direction: &Direction) {
        let Size { height, .. } = self.size;
        let Location { mut x, mut y } = self.location;

        match direction {
            //vertical
            Direction::Up => {
                y = y.saturating_sub(1);
            }
            Direction::Down => {
                y = y.saturating_add(1);
                //y = std::cmp::min(height.saturating_sub(1), y.saturating_add(1)); y = std::cmp::min(height.saturating_sub(1), y.saturating_add(1));
            }
            Direction::PageUp => {
                y = y.saturating_add(1).saturating_sub(height);
            }
            Direction::PageDown => {
                //doesn't work correctly currently since scroll offset is not preperly updated
                //i guess
                y = y.saturating_add(height);
            }
            //horizontal
            Direction::Left => {
                x = x.saturating_sub(1);
            }
            Direction::Right => {
                let len = self.buffer.lines.get(y).map_or(0, Line::grapheme_count);
                x = std::cmp::min(x.saturating_add(1), len);
            }
            Direction::Home => {
                x = 0;
            }
            Direction::End => {
                //how to handle view?
                x = self.buffer.lines.get(y).map_or(0, Line::grapheme_count);
            }
        }
        y = std::cmp::min(y, self.buffer.lines.len());
        x = self
            .buffer
            .lines
            .get(y)
            .map_or(0, |line| std::cmp::min(x, line.grapheme_count()));

        self.location = Location { x, y };
        self.update_scroll_offset();
    }

    fn text_location_to_position(&self) -> Position {
        let Location { x, y } = self.location;
        let x = self
            .buffer
            .lines
            .get(y)
            .map_or(0, |line| line.width_until(x));
        Position { col: x, row: y }
    }

    fn update_scroll_offset(&mut self) {
        let Size { width, height } = self.size;
        let Position { col: x, row: y } = self.text_location_to_position();

        let Position {
            col: mut scroll_x,
            row: mut scroll_y,
        } = self.scroll_offset;

        //currently if the view changes, just chagne the current column to be the left most
        //location
        if x >= scroll_x.saturating_add(width) {
            //scroll_x = x.saturating_add(width);
            scroll_x = x;
        } else if x < scroll_x {
            scroll_x = x;
        }

        //vertical
        //currently if the view changes, just chagne the current row to be the top most
        //location
        if y >= scroll_y.saturating_add(height) {
            //scroll_y = y.saturating_add(height);
            scroll_y = y;
        } else if y < scroll_y {
            scroll_y = y;
        }

        self.redraw();
        if !(scroll_x == self.scroll_offset.col && scroll_y == self.scroll_offset.row) {
            //doesn't matter whether x or y changes
            self.scroll_offset = Position {
                col: scroll_x,
                row: scroll_y,
            };
        }
    }

    pub fn get_caret_location(&self) -> Position {
        self.text_location_to_position()
            .subtract(&self.scroll_offset)
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Insert(char) => self.insert_char(char),
            EditorCommand::Quit => {}
        }
    }
}
