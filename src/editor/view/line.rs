use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    fn saturating_add(&self, other: usize) -> usize {
        other.saturating_add(self.to_value())
    }
    // Todo: implement From tarit?
    fn to_value(&self) -> usize {
        match self {
            Self::Half => 1,
            Self::Full => 2,
        }
    }

    fn from_value(value: usize) -> Self {
        match value {
            0 | 1 => Self::Half,
            _ => Self::Full,
        }
    }
}

pub struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
}

pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {
    fn str_to_fragments(string: &str) -> Vec<TextFragment> {
        let fragments = string
            .graphemes(true)
            .map(|grapheme| {
                let (replacement, rendered_width) = Self::replacement_character(grapheme)
                    .map_or_else(
                        //use trait
                        || (None, GraphemeWidth::from_value(grapheme.width())),
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect();
        fragments
    }
    pub fn from(string: &str) -> Self {
        let fragments = Self::str_to_fragments(string);
        Self { fragments }
    }

    fn replacement_character(str: &str) -> Option<char> {
        //TODO: https://www.unicode.org/charts/PDF/U2400.pdf
        let width = str.width();

        match str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = str.chars();
                if let Some(ch) = chars.next() {
                    //fix: control character donot work
                    if ch.is_control() && chars.next().is_none() {
                        return Some('K');
                    }
                }

                Some('.')
            }

            _ => None,
        }
    }
    pub fn get_graphemes(&self, range: Range<usize>) -> String {
        if range.start > range.end {
            return String::new();
        }
        let mut result = String::new();
        let mut curr_position = 0;
        for fragment in self.fragments.iter() {
            let fragment_end = fragment.rendered_width.saturating_add(curr_position);
            if curr_position >= range.end {
                break;
            }
            if fragment_end > range.start {
                if fragment_end > range.end || curr_position < range.start {
                    //edge case: ellipsis
                    result.push('⋯');
                } else if let Some(char) = fragment.replacement {
                    result.push(char);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }
            curr_position = fragment_end;
        }
        result
    }

    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|grapheme| grapheme.rendered_width.to_value())
            .sum()
    }

    pub fn insert_char(&mut self, char: char, insert_index: usize) {
        let mut result = String::new();
        for (index, fragment) in self.fragments.iter().enumerate() {
            if index == insert_index {
                result.push(char);
            }
            result.push_str(&fragment.grapheme);
        }
        if insert_index == self.fragments.len() {
            result.push(char);
        }

        self.fragments = Self::str_to_fragments(&result);
    }

    pub fn remove_grapheme_at(&mut self, remove_index: usize) {
        // do tutorial's way
        self.fragments.remove(remove_index);
    }
}
