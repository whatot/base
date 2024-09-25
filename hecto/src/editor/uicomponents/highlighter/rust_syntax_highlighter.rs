use std::collections::HashMap;

use crate::editor::{Annotation, AnnotationType, Line};

use super::LineIdx;

#[derive(Default)]
pub struct RustSyntaxHighlighter {
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

impl RustSyntaxHighlighter {
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
}

impl super::SyntaxHighlighter for RustSyntaxHighlighter {
    fn highlight(&mut self, idx: LineIdx, line: &Line) {
        let mut result = Vec::new();
        Self::highlight_digits(line, &mut result);
        self.highlights.insert(idx, result);
    }

    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }
}
