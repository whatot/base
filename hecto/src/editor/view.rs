use std::io::Error;

use super::{
    buffer::Buffer,
    terminal::{Position, Size, Terminal},
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    pub buffer: Buffer,
    pub need_redraw: bool,
    pub size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.need_redraw = true;
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.need_redraw {
            return Ok(());
        }

        let Size { width, height } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }

        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for cur in 0..height {
            if let Some(line) = self.buffer.lines.get(cur) {
                let truncated_line = if line.len() > width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(cur, truncated_line)?;
            } else if cur == vertical_center && self.buffer.is_empty() {
                Self::render_line(cur, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(cur, "~")?;
            }
        }

        self.need_redraw = false;
        Ok(())
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

    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position { row: at, col: 0 })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }
}
