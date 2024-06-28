use std::ops::Range;
pub struct Line {
    string: String,
}

impl Line {
    pub fn get(&self, range: Range<usize>) -> String {
        let Range { start, end } = range;
        let end = std::cmp::min(self.string.len(), end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub fn from(string: &str) -> Self {
        Line {
            string: String::from(string),
        }
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }
}
