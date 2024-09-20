use std::io::Error;

use super::{line::Line, Location};

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn load(filename: &str) -> Result<Self, Error> {
        let file_content = std::fs::read_to_string(filename)?;
        let lines = file_content.lines().map(Line::from).collect();
        Ok(Buffer { lines })
    }

    pub fn insert_char(&mut self, c: char, at: Location) {
        if at.line_index > self.lines.len() {
            return;
        }
        if at.line_index == self.lines.len() {
            self.lines.push(Line::from(&c.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(c, at.grapheme_index);
        }
    }

    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get(at.line_index) {
            if at.grapheme_index >= line.grapheme_count()
                && self.lines.len() > at.line_index.saturating_add(1)
            {
                // 行尾删除右侧的内容，把下一行的内容拼接到当前行
                let next_line = self.lines.remove(at.line_index.saturating_add(1));
                // clippy::indexing_slicing: We checked for existence of this line in the surrounding if statment
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_index].append(&next_line);
            } else if at.grapheme_index < line.grapheme_count() {
                // 行内删除右侧的内容
                // clippy::indexing_slicing: We checked for existence of this line in the surrounding if statment
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_index].delete(at.grapheme_index);
            }
        }
    }
}
