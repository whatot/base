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
        debug_assert!(at.line_idx <= self.height());
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
        } else {
            #[cfg(debug_assertions)]
            {
                panic!("Attempting to save with no file path present");
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

    pub fn search_forward(&self, query: &str, from: Location) -> Option<Location> {
        if query.is_empty() {
            return None;
        }

        // taking one more, to search the current line twice (once from the middle, once from the start)
        let mut is_first = true;
        for (line_idx, line) in self
            .lines
            .iter()
            .enumerate()
            .cycle()
            .skip(from.line_idx)
            .take(self.lines.len().saturating_add(1))
        {
            let from_grapheme_idx = if is_first {
                is_first = false;
                from.grapheme_idx
            } else {
                0
            };

            if let Some(grapheme_idx) = line.search_forward(query, from_grapheme_idx) {
                return Some(Location {
                    grapheme_idx,
                    line_idx,
                });
            }
        }

        None
    }

    pub fn search_backword(&self, query: &str, from: Location) -> Option<Location> {
        if query.is_empty() {
            return None;
        }

        let mut is_first = true;
        for (line_idx, line) in self
            .lines
            .iter()
            .enumerate()
            .rev()
            .cycle()
            .skip(
                self.lines
                    .len()
                    .saturating_sub(from.line_idx)
                    .saturating_sub(1),
            )
            .take(self.lines.len().saturating_add(1))
        {
            let from_grapheme_idx = if is_first {
                is_first = false;
                from.grapheme_idx
            } else {
                line.grapheme_count()
            };

            if let Some(grapheme_idx) = line.search_backward(query, from_grapheme_idx) {
                return Some(Location {
                    grapheme_idx,
                    line_idx,
                });
            }
        }

        None
    }
}
