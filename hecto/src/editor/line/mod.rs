use crate::prelude::*;
use std::{
    char,
    cmp::min,
    fmt,
    ops::{Deref, Range},
};

use grapheme_width::GraphemeWidth;
use text_fragment::TextFragment;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::{annotate::AnnotatedString, Annotation};

mod grapheme_width;
mod text_fragment;

#[derive(Default, Clone)]
pub struct Line {
    fragments: Vec<TextFragment>,
    string: String,
}

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
            .map(|(bype_idx, grapheme)| {
                let (replacement, rendered_width) = Self::get_replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let rendered_width = match grapheme.width() {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                    start: bype_idx,
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
        self.get_annotated_visible_substr(range, None).to_string()
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
        annotations: Option<&Vec<Annotation>>,
    ) -> AnnotatedString {
        if range.start >= range.end {
            return AnnotatedString::default();
        }

        // Create a new annotated string
        let mut result = AnnotatedString::from(&self.string);

        // Apply annotations for this string
        if let Some(annotations) = annotations {
            for annotation in annotations {
                result.add_annotation(annotation.annotation_type, annotation.start, annotation.end);
            }
        }

        result
    }

    pub fn grapheme_count(&self) -> GraphemeIdx {
        self.fragments.len()
    }

    pub fn width_until(&self, grapheme_idx: GraphemeIdx) -> ColIdx {
        self.fragments
            .iter()
            .take(grapheme_idx)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }

    pub fn width(&self) -> ColIdx {
        self.width_until(self.grapheme_count())
    }

    // Inserts a character into the line, or appends it at the end if at == grapheme_count + 1
    pub fn insert_char(&mut self, c: char, at: GraphemeIdx) {
        debug_assert!(at.saturating_sub(1) <= self.grapheme_count());
        if let Some(fragment) = self.fragments.get(at) {
            self.string.insert(fragment.start, c);
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
            let start = fragment.start;
            let end = fragment.start.saturating_add(fragment.grapheme.len());
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
            let remainer = self.string.split_off(fragment.start);
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
            .position(|fragment| fragment.start >= byte_idx)
    }

    pub fn grapheme_idx_to_byte_idx(&self, grapheme_idx: GraphemeIdx) -> ByteIdx {
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
            |fragment| fragment.start,
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
        let start = self.grapheme_idx_to_byte_idx(from_grapheme_idx);
        self.find_all(query, start..self.string.len())
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

    pub fn find_all(&self, query: &str, range: Range<ByteIdx>) -> Vec<(ByteIdx, GraphemeIdx)> {
        let end = min(range.end, self.string.len());
        let start = range.start;
        debug_assert!(start <= end);
        debug_assert!(start <= self.string.len());

        self.string.get(start..end).map_or_else(Vec::new, |substr| {
            let potential_matches: Vec<ByteIdx> = substr
                // find _potential_ matches within the substring
                .match_indices(query)
                .map(|(relative_start_idx, _)| {
                    // convert their relative indices to absolute indices
                    relative_start_idx.saturating_add(start)
                })
                .collect();

            //convert the potential matches into matches which align with the grapheme boundaries.
            self.match_graphme_clusters(&potential_matches, query)
        })
    }

    // Finds all matches which align with grapheme boundaries.
    // Parameters:
    // - query: The query to search for.
    // - matches: A vector of byte indices of potential matches, which might or might not align with the grapheme clusters.
    // Returns:
    // A Vec of (byte_index, grapheme_idx) pairs for each match that alignes with the grapheme clusters, where byte_index is the byte index of the match, and grapheme_idx is the grapheme index of the match.
    fn match_graphme_clusters(
        &self,
        matches: &[ByteIdx],
        query: &str,
    ) -> Vec<(ByteIdx, GraphemeIdx)> {
        let grapheme_count = query.graphemes(true).count();
        matches
            .iter()
            .filter_map(|&start| {
                self.byte_idx_to_grapheme_idx(start)
                    .and_then(|grapheme_idx| {
                        self.fragments
                            .get(grapheme_idx..grapheme_idx.saturating_add(grapheme_count)) // get all fragments that should be part of the match
                            .and_then(|fragments| {
                                let substring = fragments
                                    .iter()
                                    .map(|fragment| fragment.grapheme.as_str())
                                    .collect::<String>(); //combine the fragments into a single string.
                                (substring == query).then_some((start, grapheme_idx))
                                // if the combined string matches the query, we have an actual match.
                            })
                    })
            })
            .collect()
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
