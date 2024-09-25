use crate::editor::Annotation;

use super::LineIdx;
use crate::editor::Line;

pub trait SyntaxHighlighter {
    fn highlight(&mut self, idx: LineIdx, line: &Line);
    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>>;
}
