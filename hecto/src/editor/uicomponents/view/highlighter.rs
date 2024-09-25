use std::collections::HashMap;

use crate::editor::{line::Line, Annotation, AnnotationType};

use super::{LineIdx, Location};

#[derive(Default)]
pub struct Highlighter<'a> {
    match_word: Option<&'a str>,
    selected_match: Option<Location>,
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

impl<'a> Highlighter<'a> {
    pub fn new(match_word: Option<&'a str>, selected_match: Option<Location>) -> Self {
        Self {
            match_word,
            selected_match,
            highlights: HashMap::new(),
        }
    }

    pub fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }

    fn highlight_digits(line: &Line, result: &mut Vec<Annotation>) {
        line.chars().enumerate().for_each(|(idx, ch)| {
            if ch.is_ascii_digit() {
                result.push(Annotation {
                    annotation_type: AnnotationType::Digit,
                    start: idx,
                    end: idx.saturating_add(1),
                });
            }
        });
    }

    fn highlight_matched_word(&self, line: &Line, result: &mut Vec<Annotation>) {
        if let Some(matched_word) = self.match_word {
            if matched_word.is_empty() {
                return;
            }

            line.find_all(matched_word, 0..line.len())
                .iter()
                .for_each(|(start, _)| {
                    result.push(Annotation {
                        annotation_type: AnnotationType::Match,
                        start: *start,
                        end: start.saturating_add(matched_word.len()),
                    });
                });
        }
    }

    fn highlight_selected_match(&self, result: &mut Vec<Annotation>) {
        if let Some(selected_match) = self.selected_match {
            if let Some(match_word) = self.match_word {
                if match_word.is_empty() {
                    return;
                }
                let start = selected_match.grapheme_idx;
                result.push(Annotation {
                    annotation_type: AnnotationType::SelectedMatch,
                    start,
                    end: start.saturating_add(match_word.len()),
                });
            }
        }
    }

    pub fn hightlight(&mut self, idx: LineIdx, line: &Line) {
        let mut result = Vec::new();
        Self::highlight_digits(line, &mut result);
        self.highlight_matched_word(line, &mut result);
        if let Some(selected_match) = self.selected_match {
            if selected_match.line_idx == idx {
                self.highlight_selected_match(&mut result);
            }
        }
        self.highlights.insert(idx, result);
    }
}
