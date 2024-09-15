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
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
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
        let (width, height) = size()?;
        Ok(Size { width, height })
    }

    pub fn move_cursor_to(p: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(p.x, p.y))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(cursor::Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
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

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}