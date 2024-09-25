use super::{FileType, LineIdx};

#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus {
    pub total_lines: usize,
    pub current_line_idx: LineIdx,
    pub is_modified: bool,
    pub file_name: String,
    pub file_type: FileType,
}

impl DocumentStatus {
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            "(modified)".to_string()
        } else {
            String::new()
        }
    }

    pub fn line_count_to_string(&self) -> String {
        format!("{} lines", self.total_lines)
    }

    pub fn position_indicator_to_string(&self) -> String {
        format!(
            "{}:{}",
            self.current_line_idx.saturating_add(1),
            self.total_lines
        )
    }

    pub fn file_type_to_string(&self) -> String {
        self.file_type.to_string()
    }
}
