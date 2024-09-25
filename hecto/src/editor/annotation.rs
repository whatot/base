use super::AnnotationType;
use crate::prelude::*;

// clippy::struct_field_names: naming the field `type` is disallowed due to type being a keyword.
#[derive(Copy, Clone, Debug)]
#[allow(clippy::struct_field_names)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub start: ByteIdx,
    pub end: ByteIdx,
}

impl Annotation {
    pub fn shift(&mut self, shift: ByteIdx) {
        self.start = self.start.saturating_add(shift);
        self.end = self.end.saturating_add(shift);
    }
}
