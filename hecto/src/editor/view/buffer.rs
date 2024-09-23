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
        if at.line_idx > self.height() {
            return;
        }
        if at.line_idx == self.height() {
            self.lines.push(Line::from(&c.to_string()));
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_idx) {
            line.insert_char(c, at.grapheme_idx);
            self.dirty = true;
        }
    }

    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get(at.line_idx) {
            if at.grapheme_idx >= line.grapheme_count()
                && self.height() > at.line_idx.saturating_add(1)
            {
                // 行尾删除右侧的内容，把下一行的内容拼接到当前行
                let next_line = self.lines.remove(at.line_idx.saturating_add(1));
                // clippy::indexing_slicing: We checked for existence of this line in the surrounding if statment
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_idx].append(&next_line);
                self.dirty = true;
            } else if at.grapheme_idx < line.grapheme_count() {
                // 行内删除右侧的内容
                // clippy::indexing_slicing: We checked for existence of this line in the surrounding if statment
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_idx].delete(at.grapheme_idx);
                self.dirty = true;
            }
        }
    }

    pub fn insert_newline(&mut self, at: Location) {
        // 有效行之外
        if at.line_idx >= self.height() {
            self.lines.push(Line::default());
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_idx) {
            // 行首、行中、行尾
            let new_line = line.split(at.grapheme_idx);
            self.lines.insert(at.line_idx.saturating_add(1), new_line);
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

    pub fn search(&self, query: &str, from: Location) -> Option<Location> {
        // Search from within the current line until the bottom of the document
        for (line_idx, line) in self.lines.iter().enumerate().skip(from.line_idx) {
            let from_grapheme_idx = if line_idx == from.line_idx {
                //Ensure that we start in the current line from the desired position.
                from.grapheme_idx
            } else {
                //For every other line, we start at the begining of the line.
                0
            };

            if let Some(grapheme_idx) = line.search(query, from_grapheme_idx) {
                return Some(Location {
                    grapheme_idx,
                    line_idx,
                });
            }
        }

        // Search from the top of the document until (and including) the current line
        for (line_idx, line) in self.lines.iter().enumerate().take(from.line_idx) {
            if let Some(grapheme_idx) = line.search(query, 0) {
                return Some(Location {
                    grapheme_idx,
                    line_idx,
                });
            }
        }

        None
    }
}
