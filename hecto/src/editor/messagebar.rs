use std::time::{Duration, Instant};

use crate::editor::terminal::Terminal;

use super::{terminal::Size, uicomponent::UIComponent};

const DEFAULT_DURATION: Duration = Duration::new(5, 0);

struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

#[derive(Default)]
pub struct MessageBar {
    current_message: Message,
    need_redraw: bool,
    cleared_after_expiry: bool, //ensures we can properly hide expired messages
}

impl MessageBar {
    pub fn update_message(&mut self, new_msg: String) {
        self.current_message = Message {
            text: new_msg,
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.set_needs_redraw(true);
    }
}

impl UIComponent for MessageBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.need_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.need_redraw
    }

    fn set_size(&mut self, _: Size) {}

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        if self.current_message.is_expired() {
            // Upon expiration, we need to write out "" once to clear the message.
            // To avoid clearing more than necessary, we keep track of the fact that we've already cleared the expired message once.
            self.cleared_after_expiry = true;
        }

        let message = if self.current_message.is_expired() {
            ""
        } else {
            &self.current_message.text
        };

        Terminal::print_row(origin_y, message)
    }
}
