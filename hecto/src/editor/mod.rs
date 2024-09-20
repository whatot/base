use std::io::Error;

use crossterm::event::Event::{self};
use crossterm::event::KeyEvent;
use crossterm::event::{read, KeyEventKind};

mod editorcommand;
mod statusbar;
mod terminal;
mod view;
use editorcommand::EditorCommand;
use statusbar::StatusBar;
use terminal::Terminal;
use view::View;

pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
}

#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus {
    pub total_lines: usize,
    pub current_line_index: usize,
    pub is_modified: bool,
    pub file_name: Option<String>,
}

impl Editor {
    pub fn new() -> Result<Editor, Error> {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::initialize()?;

        let mut view = View::new(2);
        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            view.load(filename);
        }

        let mut status_bar = StatusBar::new(1);
        status_bar.update_status(view.get_status());

        Ok(Self {
            should_quit: false,
            view,
            status_bar,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => {
                    self.evaluate_event(event);
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }

            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if !should_process {
            return;
        }

        if let Ok(command) = EditorCommand::try_from(event) {
            if matches!(command, EditorCommand::Quit) {
                self.should_quit = true;
            } else {
                self.view.handle_command(&command);
                if let EditorCommand::Resize(size) = command {
                    self.status_bar.resize(size);
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        self.status_bar.render();
        let _ = Terminal::move_caret_to(self.view.caret_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
