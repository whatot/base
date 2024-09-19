use std::{cmp, ops::Range};

use unicode_segmentation::UnicodeSegmentation;

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        Self {
            string: String::from(line_str),
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp::min(self.len(), range.end);
        self.string
            .graphemes(true)
            .into_iter()
            .skip(start)
            .take(end - start)
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn len(&self) -> usize {
        self.string.graphemes(true).count()
    }
}
