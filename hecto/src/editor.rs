use crossterm::event::Event::Key;
use crossterm::event::KeyCode::Char;
use crossterm::{
    event::read,
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        if let Err(err) = self.repl() {
            panic!("{err:?}");
        }

        println!("Goodbye!");
    }

    fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {e}");
                    break;
                }
                _ => {}
            }
        }

        disable_raw_mode()?;
        Ok(())
    }
}
