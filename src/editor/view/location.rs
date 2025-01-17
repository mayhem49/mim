use crate::editor::terminal::Position;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<Location> for Position {
    fn from(loc: Location) -> Self {
        Self {
            col: loc.x,
            row: loc.y,
        }
    }
}
