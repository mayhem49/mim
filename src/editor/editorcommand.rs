use super::terminal::Size;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
}
pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Insert(char),
    Quit,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        #[allow(clippy::enum_glob_use)]
        use Direction::*;

        match event {
            Event::Resize(w_u16, h_u16) =>
            {
                #[allow(clippy::as_conversions)]
                Ok(Self::Resize(Size {
                    width: w_u16 as usize,
                    height: h_u16 as usize,
                }))
            }

            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Up, _) => Ok(Self::Move(Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Down)),
                (KeyCode::Right, _) => Ok(Self::Move(Right)),
                (KeyCode::Left, _) => Ok(Self::Move(Left)),
                (KeyCode::PageUp, _) => Ok(Self::Move(PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(PageDown)),
                (KeyCode::Home, _) => Ok(Self::Move(Home)),
                (KeyCode::End, _) => Ok(Self::Move(End)),
                (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::Insert(c)),
                _ => Err(format!(
                    " keycode {code:?} not implemented in editor command"
                )),
            },
            _ => Err(format!(
                "event: {event:?} not implemented in editor command"
            )),
        }
    }
}
