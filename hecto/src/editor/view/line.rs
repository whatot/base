use std::{char, ops::Range};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub struct Line {
    fragments: Vec<TextFragment>,
}

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

pub struct TextFragment {
    grapheme: String,
    rendered_width: GraphmeWidth,
    replacement: Option<char>,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        let fragments = line_str
            .graphemes(true)
            .map(|str| {
                let (replacement, rendered_width) = Self::replacement_character(str).map_or_else(
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
                }
            })
            .collect();

        Self { fragments }
    }

    fn replacement_character(for_str: &str) -> Option<char> {
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

    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
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

    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|fragment| match fragment.rendered_width {
                GraphmeWidth::Half => 1,
                GraphmeWidth::Full => 2,
            })
            .sum()
    }
}