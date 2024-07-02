use super::line::Line;
use super::Location;
use std::io::Error;
#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}
impl Buffer {
    pub fn load(file: &str) -> Result<Self, Error> {
        let data = std::fs::read_to_string(file)?;
        //let lines: Vec<Line> =
        //data.lines().map(std::string::ToString::to_string).collect();
        let mut buffer = Buffer::default();
        for line in data.lines() {
            buffer.lines.push(Line::from(line));
        }
        Ok(buffer)
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn insert_char(&mut self, char: char, text_location: Location) {
        let Location { x, y } = text_location;
        if let Some(line) = self.lines.get_mut(y) {
            line.insert_char(char, x);
        } else if y == self.lines.len() {
            self.lines.push(Line::from(&char.to_string()));
        }
    }
}
