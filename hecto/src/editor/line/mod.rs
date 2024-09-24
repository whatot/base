use std::{
    char, fmt,
    ops::{Deref, Range},
};

use grapheme_width::GraphemeWidth;
use text_fragment::TextFragment;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::annotate::{AnnotatedString, AnnotationType};

mod grapheme_width;
mod text_fragment;

#[derive(Default, Clone)]
pub struct Line {
    fragments: Vec<TextFragment>,
    string: String,
}

type GraphemeIdx = usize;
type ByteIdx = usize;
type ColIdx = usize;

impl Line {
    pub fn from(line_str: &str) -> Self {
        debug_assert!(line_str.is_empty() || line_str.lines().count() == 1);
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
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
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

    // Gets the visible graphemes in the given column index.
    // Note that the column index is not the same as the grapheme index:
    // A grapheme can have a width of 2 columns.
    pub fn get_visible_graphemes(&self, range: Range<ColIdx>) -> String {
        self.get_annotated_visible_substr(range, None, None)
            .to_string()
    }

    // Gets the annotated string in the given column index.
    // Note that the column index is not the same as the grapheme index:
    // A grapheme can have a width of 2 columns.
    // Parameters:
    // - range: The range of columns to get the annotated string from.
    // - query: The query to highlight in the annotated string.
    // - selected_match: The selected match to highlight in the annotated string. This is only applied if the query is not empty.
    pub fn get_annotated_visible_substr(
        &self,
        range: Range<ColIdx>,
        query: Option<&str>,
        selected_match: Option<GraphemeIdx>,
    ) -> AnnotatedString {
        if range.start >= range.end {
            return AnnotatedString::default();
        }
        // Create a new annotated string
        let mut result = AnnotatedString::from(&self.string);
        // Annotate it based on the search results
        if let Some(query) = query {
            if !query.is_empty() {
                self.find_all(query, 0..self.string.len()).iter().for_each(
                    |(start_byte_idx, grapheme_idx)| {
                        if let Some(selected_match) = selected_match {
                            if *grapheme_idx == selected_match {
                                result.add_annotation(
                                    AnnotationType::SelectedMatch,
                                    *start_byte_idx,
                                    start_byte_idx.saturating_add(query.len()),
                                );
                                return;
                            }
                        }
                        result.add_annotation(
                            AnnotationType::Match,
                            *start_byte_idx,
                            start_byte_idx.saturating_add(query.len()),
                        );
                    },
                );
            }
        }
        // Insert replacement characters, and truncate if needed.
        // We do this backwards, otherwise the byte indices would be off in case a replacement character has a different width than the original character.
        let mut fragment_start = self.width();
        for fragment in self.fragments.iter().rev() {
            let fragment_end = fragment_start;
            fragment_start = fragment_start.saturating_sub(fragment.rendered_width.into());
            if fragment_start > range.end {
                continue; // No  processing needed if we haven't reached the visible range yet.
            }
            // clip right if the fragment is partially visible
            if fragment_start < range.end && fragment_end > range.end {
                result.replace(fragment.start_byte_idx, self.string.len(), "⋯");
                continue;
            } else if fragment_start == range.end {
                // Truncate right if we've reached the end of the visible range
                result.replace(fragment.start_byte_idx, self.string.len(), "");
                continue;
            }
            // Fragment ends at the start of the range: Remove the entire left side of the string (if not already at start of string)
            if fragment_end <= range.start {
                result.replace(
                    0,
                    fragment
                        .start_byte_idx
                        .saturating_add(fragment.grapheme.len()),
                    "",
                );
                break; //End processing since all remaining fragments will be invisible.
            } else if fragment_start < range.start && fragment_end > range.start {
                // Fragment overlaps with the start of range: Remove the left side of the string and add an ellipsis
                result.replace(
                    0,
                    fragment
                        .start_byte_idx
                        .saturating_add(fragment.grapheme.len()),
                    "⋯",
                );
                break; //End processing since all remaining fragments will be invisible.
            }
            // Fragment is fully within range: Apply replacement characters if appropriate
            if fragment_start >= range.start && fragment_end <= range.end {
                if let Some(replacement) = fragment.replacement {
                    let start_byte_idx = fragment.start_byte_idx;
                    let end_byte_idx = start_byte_idx.saturating_add(fragment.grapheme.len());
                    result.replace(start_byte_idx, end_byte_idx, &replacement.to_string());
                }
            }
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
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }

    pub fn width(&self) -> GraphemeIdx {
        self.width_until(self.grapheme_count())
    }

    pub fn insert_char(&mut self, c: char, at: GraphemeIdx) {
        debug_assert!(at.saturating_sub(1) <= self.grapheme_count());
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
        debug_assert!(at <= self.grapheme_count());
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

    fn byte_idx_to_grapheme_idx(&self, byte_idx: ByteIdx) -> Option<GraphemeIdx> {
        if byte_idx > self.string.len() {
            return None;
        }
        self.fragments
            .iter()
            .position(|fragment| fragment.start_byte_idx >= byte_idx)
    }

    fn grapheme_idx_to_byte_idx(&self, grapheme_idx: GraphemeIdx) -> ByteIdx {
        debug_assert!(grapheme_idx <= self.grapheme_count());
        if grapheme_idx == 0 || self.grapheme_count() == 0 {
            return 0;
        }
        self.fragments.get(grapheme_idx).map_or_else(
            || {
                #[cfg(debug_assertions)]
                {
                    panic!("Fragment not found for grapheme index: {grapheme_idx:?}");
                }
                #[cfg(not(debug_assertions))]
                {
                    0
                }
            },
            |fragment| fragment.start_byte_idx,
        )
    }

    pub fn search_forward(
        &self,
        query: &str,
        from_grapheme_idx: GraphemeIdx,
    ) -> Option<GraphemeIdx> {
        debug_assert!(from_grapheme_idx <= self.grapheme_count());
        if from_grapheme_idx == self.grapheme_count() {
            return None;
        }
        let start_byte_idx = self.grapheme_idx_to_byte_idx(from_grapheme_idx);
        self.find_all(query, start_byte_idx..self.string.len())
            .first()
            .map(|(_, grapheme_idx)| *grapheme_idx)
    }

    pub fn search_backward(
        &self,
        query: &str,
        from_grapheme_idx: GraphemeIdx,
    ) -> Option<GraphemeIdx> {
        debug_assert!(from_grapheme_idx <= self.grapheme_count());
        if from_grapheme_idx == 0 {
            return None;
        }
        let end_byte_index = if from_grapheme_idx == self.grapheme_count() {
            self.string.len()
        } else {
            self.grapheme_idx_to_byte_idx(from_grapheme_idx)
        };
        self.find_all(query, 0..end_byte_index)
            .last()
            .map(|(_, grapheme_idx)| *grapheme_idx)
    }

    fn find_all(&self, query: &str, range: Range<ByteIdx>) -> Vec<(ByteIdx, GraphemeIdx)> {
        let end_byte_idx = range.end;
        let start_byte_idx = range.start;
        self.string
            .get(start_byte_idx..end_byte_idx)
            .map_or_else(Vec::new, |substr| {
                substr
                    .match_indices(query) // find _potential_ matches within the substring
                    .filter_map(|(relative_start_idx, _)| {
                        let absolute_start_idx = relative_start_idx.saturating_add(start_byte_idx); //convert their relative indices to absolute indices
                        self.byte_idx_to_grapheme_idx(absolute_start_idx)
                            .map(|grapheme_idx| (absolute_start_idx, grapheme_idx))
                    })
                    .collect()
            })
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
