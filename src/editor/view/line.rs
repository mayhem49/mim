use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
pub struct Line {
    string: String,
}

impl Line {
    pub fn get(&self, range: Range<usize>) -> String {
        let Range { start, end } = range;
        let end = std::cmp::min(self.utf_len(), end);
        self.string
            .graphemes(true)
            .skip(start)
            .take(end.saturating_sub(start))
            .collect()
    }

    pub fn from(string: &str) -> Self {
        Line {
            string: String::from(string),
        }
    }

    pub fn utf_len(&self) -> usize {
        self.string.graphemes(true).count()
    }
}
