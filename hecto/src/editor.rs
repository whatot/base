use std::io::Error;

use crossterm::cursor::MoveTo;
use crossterm::event::Event::{self, Key};
use crossterm::event::KeyCode::Char;
use crossterm::event::{KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    event::read,
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), Error> {
        let mut stdout = std::io::stdout();
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, Clear(ClearType::All))
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
            Self::clear_screen()?;
            println!("Goodbye!\r\n");
        }
        Ok(())
    }
}
