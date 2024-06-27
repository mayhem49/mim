use std::io::Error;
#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}
impl Buffer {
    pub fn load(file: &str) -> Result<Self, Error> {
        let data = std::fs::read_to_string(file)?;
        let lines: Vec<String> = data.lines().map(std::string::ToString::to_string).collect();
        Ok(Buffer { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
