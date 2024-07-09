//one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty twenty-one twenty-two twenty-three twenty-four twenty-five twenty-six twenty-seven twenty-eight twenty-nine thirty thirty-one thirty-two thirty-three thirty-four thirty-five thirty-six thirty-seven thirty-eight thirty-nine forty forty-one forty-two forty-three forty-four forty-five forty-six forty-seven forty-eight forty-nine fifty fifty-one fifty-two fifty-three fifty-four fifty-five
use super::{
    command::{Edit, Move},
    terminal::{Position, Size, Terminal},
    uicomponent::UIComponent,
    view::location::Location,
    DocumentStatus,
};
use std::io::Error;

mod buffer;
mod line;
pub mod location;

use buffer::Buffer;
use line::Line;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

//location: location in the test
//y->current line in the text
//x->current grapheme in the text
//
#[derive(Default)]
pub struct View {
    buffer: Buffer,
    redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Position,
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

    fn editor_height(&self) -> usize {
        self.size.height
    }

    fn render_line(at: usize, line: &str) -> Result<(), Error> {
        Terminal::print_row(at, line)?;
        Ok(())
    }

    pub fn load(&mut self, file: &str) -> Result<(), Error> {
        if let Ok(buffer) = Buffer::load(file) {
            self.buffer = buffer;
        }
        self.mark_redraw(true);
        Ok(())
    }

    fn insert_char(&mut self, char: char) {
        //handle enter
        let Location { y, x: _ } = self.location;

        //handle None
        let old_graphemes = self.buffer.lines.get(y).map_or(0, Line::grapheme_count);

        self.buffer.insert_char(char, self.location);
        let new_graphemes = self.buffer.lines.get(y).map_or(0, Line::grapheme_count);

        if old_graphemes != new_graphemes {
            self.handle_move_command(Move::Right);
            self.mark_redraw(true);
        }
    }
    fn delete_backward(&mut self) {
        //todo: compare without indirection(direct struct comparison)
        if self.location.x != 0 || self.location.y != 0 {
            self.handle_move_command(Move::LeftUp);
            self.delete();
        }
    }

    fn delete(&mut self) {
        //maybe simplify
        if self.buffer.is_last_line(self.location.y)
            && self.location.x
                == self
                    .buffer
                    .lines
                    .get(self.location.y)
                    .unwrap()
                    .grapheme_count()
        {
            return;
        }
        self.buffer.delete(self.location);
        self.mark_redraw(true);
    }

    fn insert_new_line(&mut self) {
        self.buffer.insert_new_line(self.location);
        self.move_down(1);
        self.move_to_start_of_line();
        self.mark_redraw(true);
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.buffer.save_file()?;
        Ok(())
    }

    //region: cursor movement
    //vertical cursor movement
    fn move_up(&mut self, line: usize) {
        self.location.y = self.location.y.saturating_sub(line);
        self.snap_horizontal();
    }

    fn move_down(&mut self, line: usize) {
        self.location.y = self.location.y.saturating_add(line);
        self.snap_vertical();
        self.snap_horizontal();
    }

    //horizontal cursor movement
    fn move_left(&mut self) {
        self.location.x = self.location.x.saturating_sub(1);
    }

    fn move_left_y(&mut self) {
        if self.location.x > 0 {
            self.location.x = self.location.x.saturating_sub(1);
        } else {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    fn move_right(&mut self) {
        let len = self
            .buffer
            .lines
            .get(self.location.y)
            .map_or(0, Line::grapheme_count);
        self.location.x = std::cmp::min(self.location.x.saturating_add(1), len);
    }

    fn move_right_y(&mut self) {
        let len = self
            .buffer
            .lines
            .get(self.location.y)
            .map_or(0, Line::grapheme_count);

        if self.location.x < len {
            self.location.x = self.location.x.saturating_add(1);
        } else {
            self.move_down(1);
            self.move_to_start_of_line();
        }
    }

    fn move_to_start_of_line(&mut self) {
        self.location.x = 0;
    }
    fn move_to_end_of_line(&mut self) {
        let y = self.location.y;
        self.location.x = self.buffer.lines.get(y).map_or(0, Line::grapheme_count);
    }

    // cursor snapping
    fn snap_horizontal(&mut self) {
        let len = self
            .buffer
            .lines
            .get(self.location.y)
            .map_or(0, Line::grapheme_count);

        self.location.x = std::cmp::min(len, self.location.x);
    }

    fn snap_vertical(&mut self) {
        self.location.y = std::cmp::min(self.location.y, self.buffer.lines.len());
    }
    //end region: cursor movement

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
        let editor_height = self.editor_height();
        let Size { width, .. } = self.size;
        let Position { col: x, row: y } = self.text_location_to_position();

        let Position {
            col: mut scroll_x,
            row: mut scroll_y,
        } = self.scroll_offset;

        //currently if the view changes,
        //just chagne the current column to be the left most location
        if x >= scroll_x.saturating_add(width) {
            //scroll_x = x.saturating_add(width);
            scroll_x = x;
        } else if x < scroll_x {
            scroll_x = x;
        }

        //vertical
        //currently if the view changes,
        //just chagne the current row to be the top most location
        if y >= scroll_y.saturating_add(editor_height) {
            //scroll_y = y.saturating_add(editor_height);
            scroll_y = y;
        } else if y < scroll_y {
            scroll_y = y;
        }

        self.mark_redraw(true);
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

    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            is_modified: self.buffer.is_modified,
            curr_location: self.location,
            //why clone in every rerender
            filename: self.buffer.filename.clone(),
        }
    }

    pub fn is_modified(&self) -> bool {
        self.buffer.is_modified
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
    /// move_* command changes the `location` field only.
    /// move_* also donot do cursor spanning on neither horizontal or vertical direction.
    /// for scrolling need to call corresponding function.
    pub fn handle_move_command(&mut self, direction: Move) {
        #[allow(clippy::enum_glob_use)]
        use Move::*;
        match direction {
            //vertical
            Up => self.move_up(1),
            Down => self.move_down(1),
            PageUp => self.move_up(self.editor_height().saturating_sub(1)),
            PageDown => self.move_down(self.editor_height().saturating_sub(1)),
            //horizontal
            Left => self.move_left(),
            LeftUp => self.move_left_y(),
            RightUp => self.move_right_y(),
            Right => self.move_right(),
            StartOfLine => self.move_to_start_of_line(),
            EndOfLine => self.move_to_end_of_line(),
        }
        self.update_scroll_offset();
    }

    pub fn handle_edit_command(&mut self, command: Edit) {
        #[allow(clippy::enum_glob_use)]
        use Edit::*;
        match command {
            Insert(char) => self.insert_char(char),
            InsertNewLine => self.insert_new_line(),
            Delete => self.delete(),
            DeleteBackward => self.delete_backward(),
        }
    }
}

impl UIComponent for View {
    fn mark_redraw(&mut self, redraw: bool) {
        self.redraw = redraw;
    }

    fn needs_redraw(&self) -> bool {
        self.redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.update_scroll_offset();
    }

    fn draw(&self, start_y: usize) -> Result<(), Error> {
        let Size { width, height } = self.size;
        let end_y = start_y.saturating_add(height);

        let Position {
            col: scroll_x,
            row: scroll_y,
        } = self.scroll_offset;

        #[allow(clippy::integer_division)]
        let vertical_center = start_y.saturating_add(height / 3);

        for current_row in start_y..end_y {
            #[allow(clippy::integer_division)]
            let line_index = current_row.saturating_sub(start_y).saturating_add(scroll_y);
            if let Some(line) = self.buffer.lines.get(line_index) {
                //not utf compliant?
                let left = scroll_x;
                let right = scroll_x.saturating_add(width);
                Self::render_line(current_row, &line.get_graphemes(left..right))?;
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }

        Ok(())
    }
}
