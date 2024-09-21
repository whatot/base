use crate::editor::terminal::Terminal;

use super::{terminal::Size, uicomponent::UIComponent};

#[derive(Default)]
pub struct MessageBar {
    pub current_message: String,
    pub need_redraw: bool,
}

impl MessageBar {
    pub fn update_message(&mut self, new_msg: String) {
        if new_msg != self.current_message {
            self.current_message = new_msg;
            self.mark_redraw(true);
        }
    }
}

impl UIComponent for MessageBar {
    fn mark_redraw(&mut self, value: bool) {
        self.need_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.need_redraw
    }

    fn set_size(&mut self, _: Size) {}

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        Terminal::print_row(origin_y, &self.current_message)
    }
}
