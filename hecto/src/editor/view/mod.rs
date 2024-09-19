mod buffer;
mod line;
mod location;

use std::cmp::min;

use crate::editor::terminal::{Size, Terminal};
use buffer::Buffer;
use line::Line;
use location::Location;

use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::Position,
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    need_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}

impl View {
    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
            self.need_redraw = true;
        }
    }

    pub fn get_position(&self) -> Position {
        self.location.substract(&self.scroll_offset).into()
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
        self.need_redraw = true;
    }

    pub fn render(&mut self) {
        if !self.need_redraw {
            return;
        }

        let Size { width, height } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        let top = self.scroll_offset.y;
        let left = self.scroll_offset.x;

        for cur in 0..height {
            if let Some(line) = self.buffer.lines.get(cur.saturating_add(top)) {
                let right = left.saturating_add(width);
                Self::render_line(cur, &line.get(left..right));
            } else if cur == vertical_center && self.buffer.is_empty() {
                Self::render_line(cur, &Self::build_welcome_message(width));
            } else {
                Self::render_line(cur, "~");
            }
        }

        self.need_redraw = false;
    }

    pub fn handle_command(&mut self, command: &EditorCommand) {
        match command {
            EditorCommand::Resize(size) => self.resize(*size),
            EditorCommand::Move(direction) => self.move_text_location(direction),
            EditorCommand::Quit => (),
        }
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, .. } = self.size;

        match direction {
            Direction::Up => y = y.saturating_sub(1),
            Direction::Down => y = y.saturating_add(1),
            Direction::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    x = self.buffer.lines.get(y).map_or(0, Line::len);
                }
            }
            Direction::Right => {
                let line_width = self.buffer.lines.get(y).map_or(0, Line::len);
                if x < line_width {
                    x += 1;
                } else {
                    x = 0;
                    y = y.saturating_add(1);
                }
            }
            Direction::PageUp => y = y.saturating_sub(height).saturating_sub(1),
            Direction::PageDown => y = y.saturating_add(height).saturating_sub(1),
            Direction::Home => x = 0,
            Direction::End => x = self.buffer.lines.get(y).map_or(0, Line::len),
        }

        // snap x to valid position
        x = self
            .buffer
            .lines
            .get(y)
            .map_or(0, |line| min(line.len(), x));

        // snap y to valid position
        y = min(y, self.buffer.lines.len());

        self.location = Location { x, y };
        self.scroll_location_into_view();
    }

    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        // Scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        // Scroll horizontally
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }

        self.need_redraw = offset_changed;
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }

        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;
        let spaces = " ".repeat(padding);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        welcome_message
    }

    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }
}
