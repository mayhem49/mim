use super::line::Line;
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
}
