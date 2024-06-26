use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    Command,
};
use std::{
    fmt::Display,
    io::{stdout, Error, Write},
};
#[derive(Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
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
    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn move_cursor(Position { x, y }: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(x as u16, y as u16))?;
        Ok(())
    }

    //pub fn move_cursor_dir(dir: MoveCursor) -> Result<(), Error> {
        //let Size{width, height} = Self::size()?;
        //let Position { x, y } = Self::c
        //match dir {
            //MoveCursor::Left => Self::queue_command(MoveLeft(1))?,
            //MoveCursor::Right => Self::queue_command(MoveLeft(1))?,
            //MoveCursor::Up => Self::queue_command(MoveLeft(1))?,
            //MoveCursor::Down => Self::queue_command(MoveLeft(1))?,
        //}
        //Ok(())
    //}

    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        Ok(Size {
            width: width_u16 as usize,
            height: height_u16 as usize,
        })
    }

    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
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
}

//Terminal::clear_line()?;
//#[allow(clippy::integer_division)]
//if current_row == height / 3 {
//    Editor::draw_welcome_message()?;
//} else {
//    Editor::draw_empty_row()?;
//}
//if current_row.saturating_add(1) < height {
//    Terminal::print("\r\n")?;
//}
//
//Up => {
//    let Location { x, y } = self.cursor;
//    let y = y.saturating_sub(1);
//    //let y = std::cmp::min(y, 0);
//    self.cursor.x = x;
//    self.cursor.y = y;
//    //Terminal::move_cursor_dir(MoveCursor::Up)?,
//}
//Down => {
//    let Size { width, height } = Terminal::size()?;
//    let Location { x, y } = self.cursor;
//    //crossterm uses 0 indexing so height - 1
//    let y = std::cmp::min(y + 1, height.saturating_sub(1));
//    self.cursor.x = x;
//    self.cursor.y = y;
//    //Terminal::move_cursor_dir(MoveCursor::Up)?,
//}
////Left => Terminal::move_cursor_dir(MoveCursor::Left)?,
////Right => Terminal::move_cursor_dir(MoveCursor::Right)?,
