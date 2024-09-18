use std::io::Error;

use super::{
    buffer::Buffer,
    terminal::{Size, Terminal},
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    pub buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        #[allow(clippy::integer_division)]
        let welcome_line = height / 3;

        for cur in 0..height {
            Terminal::clear_line()?;

            if let Some(value) = self.buffer.lines.get(cur) {
                Terminal::print(value)?;
            } else if cur == welcome_line && self.buffer.lines.is_empty() {
                Self::drow_welcome_message()?;
            } else {
                Self::drow_empty_row()?;
            }

            if cur.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn drow_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let length = welcome_message.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(length)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces} {welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn drow_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
