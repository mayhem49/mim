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

    pub fn is_last_line(&self, y: usize) -> bool {
        self.lines.len().saturating_sub(1) == y
    }

    /*
     * if: bottom right or beyond => do nothing
     * else if: end of the line => concat line
     * else: delete grapheme at current cursor position;
     */
    pub fn delete(&mut self, location: Location) {
        let Location { x, y } = location;

        let is_end_of_line = x == self.lines.get(y).map_or(0, Line::grapheme_count);
        let is_last_line = self.is_last_line(y);

        //beyond bottom right
        if y >= self.lines.len() {
            return;
        }
        // bottom right
        if is_last_line && is_end_of_line {
            return;
        }

        if is_end_of_line {
            //remove and get works because it isn't last line and beyond if it is eol
            let removed_line = self.lines.remove(y.saturating_add(1));
            let line = self.lines.get_mut(y).unwrap();
            line.concat(&removed_line);
        } else if let Some(line) = self.lines.get_mut(y) {
            line.remove_grapheme_at(x);
        }
    }
}
