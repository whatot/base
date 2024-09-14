use std::io::Error;

use crossterm::event::read;
use crossterm::event::Event::{self, Key};
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyEvent, KeyModifiers};

mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code: Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) = event
        {
            self.should_quit = true;
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Goodbye!\r\n");
        } else {
            Self::drow_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }

    fn drow_rows() -> Result<(), Error> {
        let (_, height) = Terminal::size()?;
        for i in 0..height {
            print!("~");
            if i + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }
}
