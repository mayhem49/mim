use super::terminal::Size;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

//to handle unconstructed warnings
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
    LeftUp,
    RightUp,
    StartOfLine,
    EndOfLine,
    PageUp,
    PageDown,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        #[allow(clippy::enum_glob_use)]
        use Move::*;

        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers != KeyModifiers::NONE {
            return Err(format!("no corresponding move command for {event:?}"));
        }
        match code {
            KeyCode::Up => Ok(Up),
            KeyCode::Down => Ok(Down),
            KeyCode::Right => Ok(Right),
            KeyCode::Left => Ok(Left),
            KeyCode::PageUp => Ok(PageUp),
            KeyCode::PageDown => Ok(PageDown),
            KeyCode::Home => Ok(StartOfLine),
            KeyCode::End => Ok(EndOfLine),
            _ => Err(format!("no corresponding move command for {code:?}")),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Edit {
    Insert(char),
    InsertNewLine,
    Delete,
    DeleteBackward,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        #[allow(clippy::enum_glob_use)]
        use Edit::*;

        let KeyEvent {
            code, modifiers, ..
        } = event;
        match (code, modifiers) {
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Insert(c)),
            (KeyCode::Delete, KeyModifiers::NONE) => Ok(Delete),
            (KeyCode::Backspace, KeyModifiers::NONE) => Ok(DeleteBackward),
            (KeyCode::Enter, KeyModifiers::NONE) => Ok(InsertNewLine),
            (KeyCode::Tab, KeyModifiers::NONE) => Ok(Insert('\t')),
            _ => Err(format!("no corresponding edit command for {code:?}")),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Action {
    Save,
    Quit,
    ForceQuit,
    Dismiss,
    Resize(Size),
}

impl TryFrom<KeyEvent> for Action {
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        #[allow(clippy::enum_glob_use)]
        use Action::*;

        let KeyEvent {
            code, modifiers, ..
        } = event;
        match (code, modifiers) {
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Quit),
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => Ok(ForceQuit),
            (KeyCode::Char('s' | 'o'), KeyModifiers::CONTROL) => Ok(Save),
            (KeyCode::Esc, KeyModifiers::NONE) => Ok(Dismiss),
            _ => Err(format!("no corresponding action command for {event:?}")),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    Action(Action),
}

#[allow(clippy::as_conversions)]
impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Resize(w_u16, h_u16) => Ok(Self::Action(Action::Resize(Size {
                width: w_u16 as usize,
                height: h_u16 as usize,
            }))),
            Event::Key(key_event) => Edit::try_from(key_event)
                .map(Command::Edit)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| Action::try_from(key_event).map(Command::Action))
                .map_err(|_| format!("no corresponding command for {event:?}")),
            _ => Err(format!("no corresponding action command for {event:?}")),
        }
    }
}
