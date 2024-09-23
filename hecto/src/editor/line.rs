use std::{
    char, fmt,
    ops::{Deref, Range},
};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Default, Clone)]
pub struct Line {
    fragments: Vec<TextFragment>,
    string: String,
}

type GraphemeIdx = usize;
type ByteIdx = usize;

#[derive(Copy, Clone)]
pub enum GraphmeWidth {
    Half,
    Full,
}

impl GraphmeWidth {
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Half => other.saturating_add(1),
            Self::Full => other.saturating_add(2),
        }
    }
}

#[derive(Clone)]
pub struct TextFragment {
    grapheme: String,
    rendered_width: GraphmeWidth,
    replacement: Option<char>,
    start_byte_idx: usize,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Self {
            fragments,
            string: line_str.to_string(),
        }
    }

    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
            .grapheme_indices(true)
            .map(|(bype_idx, str)| {
                let (replacement, rendered_width) = Self::get_replacement_character(str)
                    .map_or_else(
                        || {
                            let rendered_width = match str.width() {
                                0 | 1 => GraphmeWidth::Half,
                                _ => GraphmeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphmeWidth::Half),
                    );

                TextFragment {
                    grapheme: str.to_string(),
                    rendered_width,
                    replacement,
                    start_byte_idx: bype_idx,
                }
            })
            .collect()
    }

    fn get_replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.len();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }
                Some('·')
            }
            _ => None,
        }
    }

    fn rebuild_fragments(&mut self) {
        self.fragments = Self::str_to_fragments(&self.string);
    }

    pub fn get_visible_graphemes(&self, range: Range<GraphemeIdx>) -> String {
        if range.end <= range.start {
            return String::new();
        }

        let mut result = String::new();
        let mut cur = 0;

        for fragment in &self.fragments {
            if cur >= range.end {
                break;
            }
            let fragment_end = fragment.rendered_width.saturating_add(cur);
            if fragment_end > range.start {
                if fragment_end > range.end || cur < range.start {
                    // Clip on the right or left
                    result.push('⋯');
                } else if let Some(char) = fragment.replacement {
                    result.push(char);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }
            cur = fragment_end;
        }

        result
    }

    pub fn grapheme_count(&self) -> GraphemeIdx {
        self.fragments.len()
    }

    pub fn width_until(&self, grapheme_idx: GraphemeIdx) -> GraphemeIdx {
        self.fragments
            .iter()
            .take(grapheme_idx)
            .map(|fragment| match fragment.rendered_width {
                GraphmeWidth::Half => 1,
                GraphmeWidth::Full => 2,
            })
            .sum()
    }

    pub fn width(&self) -> GraphemeIdx {
        self.width_until(self.grapheme_count())
    }

    pub fn insert_char(&mut self, c: char, at: GraphemeIdx) {
        if let Some(fragment) = self.fragments.get(at) {
            self.string.insert(fragment.start_byte_idx, c);
        } else {
            self.string.push(c);
        }
        self.rebuild_fragments();
    }

    pub fn append_char(&mut self, c: char) {
        self.insert_char(c, self.grapheme_count());
    }

    pub fn delete(&mut self, at: GraphemeIdx) {
        if let Some(fragment) = self.fragments.get(at) {
            let start = fragment.start_byte_idx;
            let end = fragment
                .start_byte_idx
                .saturating_add(fragment.grapheme.len());
            self.string.drain(start..end);
            self.rebuild_fragments();
        }
    }

    pub fn delete_last(&mut self) {
        self.delete(self.grapheme_count().saturating_sub(1));
    }

    pub fn append(&mut self, other: &Self) {
        self.string.push_str(&other.to_string());
        self.rebuild_fragments();
    }

    pub fn split(&mut self, at: GraphemeIdx) -> Self {
        if let Some(fragment) = self.fragments.get(at) {
            let remainer = self.string.split_off(fragment.start_byte_idx);
            self.rebuild_fragments();
            Self::from(&remainer)
        } else {
            Self::default()
        }
    }

    fn byte_idx_to_grapheme_idx(&self, byte_idx: ByteIdx) -> GraphemeIdx {
        self.fragments
            .iter()
            .position(|fragment| fragment.start_byte_idx >= byte_idx)
            .map_or(0, |grapheme_idx| grapheme_idx)
    }

    fn grapheme_idx_to_byte_idx(&self, grapheme_idx: GraphemeIdx) -> ByteIdx {
        self.fragments
            .get(grapheme_idx)
            .map_or(0, |fragment| fragment.start_byte_idx)
    }

    pub fn search(&self, query: &str, from_grapheme_idx: GraphemeIdx) -> Option<GraphemeIdx> {
        let start_byte_idx = self.grapheme_idx_to_byte_idx(from_grapheme_idx);
        self.string
            .get(start_byte_idx..)
            .and_then(|sub_str| sub_str.find(query))
            .map(|byte_idx| self.byte_idx_to_grapheme_idx(byte_idx.saturating_add(start_byte_idx)))
    }
}

impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.string)
    }
}

impl Deref for Line {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}
