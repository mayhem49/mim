use super::line::Line;
use super::Location;

use std::fs::OpenOptions;
use std::io::Error;
use std::io::Write;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub filename: Option<String>,
    pub is_modified: bool,
}
impl Buffer {
    pub fn load(filename: &str) -> Result<Self, Error> {
        if std::path::Path::new(filename).exists() {
            let data = std::fs::read_to_string(filename)?;
            let mut buffer = Buffer {
                filename: Some(String::from(filename)),
                ..Self::default()
            };
            for line in data.lines() {
                buffer.lines.push(Line::from(line));
            }
            Ok(buffer)
        } else {
            let buffer = Buffer {
                filename: Some(String::from(filename)),
                ..Self::default()
            };
            Ok(buffer)
        }
    }

    pub fn save_file(&mut self) -> Result<(), Error> {
        //do nothing if filename doesnot exist
        if self.filename.is_none() {
            return Ok(());
        }
        let mut fileptr = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.filename.as_ref().unwrap())
            .expect("couldn't open file");

        //just build a string for now
        let mut content = String::new();
        for line in &self.lines {
            content.push_str(&line.to_string());
            content.push('\n'); // write \r\n to windows(maybe autodetect)
        }

        fileptr.write_all(content.as_bytes())?;
        self.is_modified = false;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn insert_char(&mut self, char: char, text_location: Location) {
        let Location { x, y } = text_location;
        if let Some(line) = self.lines.get_mut(y) {
            line.insert_char(char, x);
            self.is_modified = true;
        } else if y == self.lines.len() {
            self.lines.push(Line::from(&char.to_string()));
            self.is_modified = true;
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
            self.is_modified = true;
        } else if let Some(line) = self.lines.get_mut(y) {
            line.remove_grapheme_at(x);
            self.is_modified = true;
        }
    }

    pub fn insert_new_line(&mut self, location: Location) {
        if let Some(line) = self.lines.get_mut(location.y) {
            let new_line = line.split_off(location.x);
            self.lines.insert(location.y.saturating_add(1), new_line);
            self.is_modified = true;
        } else if location.y == self.lines.len() {
            self.lines.push(Line::default());
            self.is_modified = true;
        } else {
            self.lines
                .insert(location.y.saturating_add(1), Line::default());
            self.is_modified = true;
        }
    }
}
