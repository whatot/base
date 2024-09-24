use crate::prelude::*;
use std::cmp::min;

use super::{annotated_string_part::AnnotatedStringPart, AnnotatedString};

pub struct AnnotatedStringIterator<'a> {
    pub annotated_string: &'a AnnotatedString,
    pub current_idx: ByteIdx,
}

impl<'a> Iterator for AnnotatedStringIterator<'a> {
    type Item = AnnotatedStringPart<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.annotated_string.string.len() {
            return None;
        }

        // Find the current active annotation
        if let Some(annotation) = self
            .annotated_string
            .annotations
            .iter()
            .filter(|annotation| {
                annotation.start <= self.current_idx && annotation.end > self.current_idx
            })
            .last()
        {
            let end_idx = min(annotation.end, self.annotated_string.string.len());
            let start_idx = self.current_idx;
            self.current_idx = end_idx;
            return Some(AnnotatedStringPart {
                string: &self.annotated_string.string[start_idx..end_idx],
                annotation_type: Some(annotation.annotation_type),
            });
        }

        // Find the boundary of the nearest annotation
        let mut end_idx = self.annotated_string.string.len();
        for annotation in &self.annotated_string.annotations {
            if annotation.start > self.current_idx && annotation.start < end_idx {
                end_idx = annotation.start;
            }
        }
        let start_idx = self.current_idx;
        self.current_idx = end_idx;

        Some(AnnotatedStringPart {
            string: &self.annotated_string.string[start_idx..end_idx],
            annotation_type: None,
        })
    }
}
