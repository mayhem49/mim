use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::{Attribute, Print, SetAttribute},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, SetTitle,
    },
    Command,
};
use std::io::{stdout, Error, Write};
#[derive(Default, Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Default, Copy, Clone)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            col: self.col.saturating_sub(other.col),
            row: self.row.saturating_sub(other.row),
        }
    }
}
pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Self::show_caret()?;
        Self::leave_alternate_screen()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }
    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn move_caret(Position { row, col }: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(col as u16, row as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        Ok(Size {
            width: width_u16 as usize,
            height: height_u16 as usize,
        })
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }
    pub fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn print_row(row: usize, line: &str) -> Result<(), Error> {
        Self::move_caret(Position { col: 0, row })?;
        Self::clear_line()?;
        Self::print(line)?;
        Ok(())
    }

    pub fn print_inverted_row(row: usize, line: &str) -> Result<(), Error> {
        Self::invert_color()?;
        Self::print_row(row, line)?;
        Self::reset_color()?;
        Ok(())
    }

    pub fn invert_color() -> Result<(), Error> {
        Self::queue_command(SetAttribute(Attribute::Reverse))?;
        Ok(())
    }
    pub fn reset_color() -> Result<(), Error> {
        Self::queue_command(SetAttribute(Attribute::Reset))?;
        Ok(())
    }

    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_command(SetTitle(title))?;
        Ok(())
    }
}
