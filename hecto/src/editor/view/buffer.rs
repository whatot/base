use std::{fs::File, io::Error};

use std::io::Write;

use super::{line::Line, Location};

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
    file_name: Option<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn load(file_name: &str) -> Result<Self, Error> {
        let file_content = std::fs::read_to_string(file_name)?;
        let lines = file_content.lines().map(Line::from).collect();
        Ok(Buffer {
            lines,
            file_name: Some(file_name.to_string()),
        })
    }

    pub fn insert_char(&mut self, c: char, at: Location) {
        if at.line_index > self.height() {
            return;
        }
        if at.line_index == self.height() {
            self.lines.push(Line::from(&c.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(c, at.grapheme_index);
        }
    }

    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get(at.line_index) {
            if at.grapheme_index >= line.grapheme_count()
                && self.height() > at.line_index.saturating_add(1)
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

    pub fn insert_newline(&mut self, at: Location) {
        // 有效行之外
        if at.line_index >= self.height() {
            self.lines.push(Line::default());
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            // 行首、行中、行尾
            let new_line = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new_line);
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = File::create(file_name)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
        }
        Ok(())
    }
}
