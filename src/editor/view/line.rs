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

    pub fn from(string: &str) -> Self {
        let fragments = string
            .graphemes(true)
            .map(|grapheme| {
                let unicode_width = grapheme.width();
                let rendered_width = match unicode_width {
                    0 | 1 => GraphemeWidth::Half,
                    _ => GraphemeWidth::Full,
                };

                let replacement = match unicode_width {
                    0 => Some('.'),
                    _ => None,
                };
                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect();
        Self { fragments }
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
}
