use std::io::Error;

use crossterm::event::Event::Key;
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

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!("Code:{code:?} Modifiers:{modifiers:?} Kind:{kind:?} State:{state:?} \r");
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => {}
                }
            }

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
        execute!(stdout, Clear(ClearType::All))
    }
}
