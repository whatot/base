use std::{
    fmt::Display,
    io::{stdout, Error, Write},
};

use crossterm::{
    cursor::{self, MoveTo},
    queue, style,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    Command,
};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        Ok(Size {
            width: width_u16 as usize,
            height: height_u16 as usize,
        })
    }

    pub fn move_caret_to(p: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(p.col as u16, p.row as u16))?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(cursor::Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(cursor::Show)?;
        Ok(())
    }

    pub fn print(t: impl Display) -> Result<(), Error> {
        Self::queue_command(style::Print(t))?;
        Ok(())
    }

    pub fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}

#[derive(Copy, Clone, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}
