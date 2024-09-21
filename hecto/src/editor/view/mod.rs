mod buffer;
mod line;

use std::{cmp::min, io::Error};

use crate::editor::terminal::{Size, Terminal};
use buffer::Buffer;
use line::Line;

use super::{
    documentstatus::DocumentStatus,
    editorcommand::{Direction, EditorCommand},
    terminal::Position,
    uicomponent::UIComponent,
    NAME, VERSION,
};

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    need_redraw: bool,
    // The view always starts at `(0/0)`. The `size` property determines the visible area.
    size: Size,
    text_location: Location,
    scroll_offset: Position,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

impl View {
    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            total_lines: self.buffer.height(),
            current_line_index: self.text_location.line_index,
            is_modified: self.buffer.dirty,
            file_name: format!("{}", self.buffer.file_info),
        }
    }

    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
            self.set_needs_redraw(true);
        }
    }

    pub fn handle_command(&mut self, command: &EditorCommand) {
        match command {
            EditorCommand::Resize(_) | EditorCommand::Quit => (),
            EditorCommand::Move(direction) => self.move_text_location(*direction),
            EditorCommand::Insert(c) => self.insert_char(*c),
            EditorCommand::Backspace => self.delete_backward(),
            EditorCommand::Delete => self.delete(),
            EditorCommand::Tab => self.insert_char('\t'),
            EditorCommand::Enter => self.insert_newline(),
            EditorCommand::Save => self.save(),
        }
    }

    fn save(&mut self) {
        let _ = self.buffer.save();
    }

    fn insert_char(&mut self, c: char) {
        let old_line_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        self.buffer.insert_char(c, self.text_location);

        let new_line_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        let grapheme_delta = new_line_len.saturating_sub(old_line_len);
        if grapheme_delta > 0 {
            self.move_text_location(Direction::Right);
        }
        self.set_needs_redraw(true);
    }

    // 删除光标左边的内容
    fn delete_backward(&mut self) {
        if self.text_location.grapheme_index != 0 || self.text_location.line_index != 0 {
            self.move_text_location(Direction::Left);
            self.delete();
        }
    }

    // 删除光标右边的内容
    fn delete(&mut self) {
        self.buffer.delete(self.text_location);
        self.set_needs_redraw(true);
    }

    fn insert_newline(&mut self) {
        self.buffer.insert_newline(self.text_location);
        self.move_text_location(Direction::Right);
        self.set_needs_redraw(true);
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::new();
        }

        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        let remaining_width = width.saturating_sub(1);
        // hide the welcome message if it doesn't fit entirely.
        if remaining_width < len {
            return "~".to_string();
        }
        format!("{:<1}{:^remaining_width$}", "~", welcome_message)
    }

    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::print_row(at, line_text)
    }

    fn scroll_vertically(&mut self, to: usize) {
        let Size { height, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };
        if offset_changed {
            self.set_needs_redraw(true);
        }
    }

    fn scroll_horizontally(&mut self, to: usize) {
        let Size { width, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };
        if offset_changed {
            self.set_needs_redraw(true);
        }
    }

    fn scroll_text_location_into_view(&mut self) {
        let Position { col, row } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    pub fn caret_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let col = self.buffer.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        Position { col, row }
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn move_text_location(&mut self, direction: Direction) {
        let Size { height, .. } = self.size;

        match direction {
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::PageUp => self.move_up(height.saturating_sub(1)),
            Direction::PageDown => self.move_down(height.saturating_sub(1)),
            Direction::Home => self.move_to_start_of_line(),
            Direction::End => self.move_to_end_of_line(),
        }
        self.scroll_text_location_into_view();
    }

    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    // clippy::arithmetic_side_effects: This function performs arithmetic calculations
    // after explicitly checking that the target value will be within bounds.
    #[allow(clippy::arithmetic_side_effects)]
    fn move_right(&mut self) {
        let line_width = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }

    // clippy::arithmetic_side_effects: This function performs arithmetic calculations
    // after explicitly checking that the target value will be within bounds.
    #[allow(clippy::arithmetic_side_effects)]
    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }
    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
    }

    // Ensures self.location.grapheme_index points to a valid grapheme index by snapping it to the left most grapheme if appropriate.
    // Doesn't trigger scrolling.
    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }
    // Ensures self.location.line_index points to a valid line index by snapping it to the bottom most line if appropriate.
    // Doesn't trigger scrolling.
    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.height());
    }
}

impl UIComponent for View {
    fn set_needs_redraw(&mut self, value: bool) {
        self.need_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.need_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_text_location_into_view();
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        let Size { width, height } = self.size;
        let end_y = origin_y.saturating_add(height);

        #[allow(clippy::integer_division)]
        let top_third = height / 3;

        let scroll_top = self.scroll_offset.row;
        let left = self.scroll_offset.col;

        for cur in origin_y..end_y {
            // to get the correct line index, we have to take current_row (the absolute row on screen),
            // subtract origin_y to get the current row relative to the view (ranging from 0 to self.size.height)
            // and add the scroll offset.
            let line_idx = cur.saturating_sub(origin_y).saturating_add(scroll_top);
            if let Some(line) = self.buffer.lines.get(line_idx) {
                let right = left.saturating_add(width);
                Self::render_line(cur, &line.get_visible_graphemes(left..right))?;
            } else if cur == top_third && self.buffer.is_empty() {
                Self::render_line(cur, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(cur, "~")?;
            }
        }
        Ok(())
    }
}
