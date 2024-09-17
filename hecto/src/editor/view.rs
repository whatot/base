use std::io::Error;

use super::terminal::{Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    pub fn drow_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for cur in 0..height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if cur == height / 3 {
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
        Terminal::print(welcome_message)?;
        Ok(())
    }

    fn drow_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
