use std::{fs::File, io::Error};

use std::io::Write;

use crate::editor::line::Line;

use super::fileinfo::FileInfo;
use super::Location;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub file_info: FileInfo,
    pub dirty: bool,
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
            file_info: FileInfo::from(file_name),
            dirty: false,
        })
    }

    pub fn insert_char(&mut self, c: char, at: Location) {
        if at.line_index > self.height() {
            return;
        }
        if at.line_index == self.height() {
            self.lines.push(Line::from(&c.to_string()));
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(c, at.grapheme_index);
            self.dirty = true;
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
                self.dirty = true;
            } else if at.grapheme_index < line.grapheme_count() {
                // 行内删除右侧的内容
                // clippy::indexing_slicing: We checked for existence of this line in the surrounding if statment
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_index].delete(at.grapheme_index);
                self.dirty = true;
            }
        }
    }

    pub fn insert_newline(&mut self, at: Location) {
        // 有效行之外
        if at.line_index >= self.height() {
            self.lines.push(Line::default());
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            // 行首、行中、行尾
            let new_line = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new_line);
            self.dirty = true;
        }
    }

    fn save_to_file(&self, file_info: &FileInfo) -> Result<(), Error> {
        if let Some(path) = &file_info.path {
            let mut file = File::create(path)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
        }

        Ok(())
    }

    pub fn is_file_loaded(&self) -> bool {
        self.file_info.has_path()
    }

    pub fn save_as(&mut self, name: &str) -> Result<(), Error> {
        let file_info = FileInfo::from(name);
        self.save_to_file(&file_info)?;
        self.file_info = file_info;
        self.dirty = false;
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.save_to_file(&self.file_info)?;
        self.dirty = false;
        Ok(())
    }

    pub fn search(&self, query: &str) -> Option<Location> {
        for (line_idx, line) in self.lines.iter().enumerate() {
            if let Some(grapheme_idx) = line.search(query) {
                return Some(Location {
                    line_index: line_idx,
                    grapheme_index: grapheme_idx,
                });
            }
        }
        None
    }
}
