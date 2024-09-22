use std::cmp::min;

use crate::editor::terminal::Terminal;

use super::{command::Edit, line::Line, size::Size, uicomponent::UIComponent};

#[derive(Default)]
pub struct CommandBar {
    prompt: String,
    value: Line,
    need_redraw: bool,
    size: Size,
}

impl CommandBar {
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(c) => self.value.append_char(c),
            Edit::Delete | Edit::InsertNewline => (),
            Edit::DeleteBackward => self.value.delete_last(),
        }
        self.set_needs_redraw(true);
    }

    pub fn caret_position_col(&self) -> usize {
        let max_width = self
            .prompt
            .len()
            .saturating_add(self.value.grapheme_count());
        min(max_width, self.size.width)
    }

    pub fn value(&self) -> String {
        self.value.to_string()
    }

    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
        self.set_needs_redraw(true);
    }

    pub fn clear_value(&mut self) {
        self.value = Line::default();
        self.set_needs_redraw(true);
    }
}

impl UIComponent for CommandBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.need_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.need_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, origin: usize) -> Result<(), std::io::Error> {
        //this is how much space there is between the right side of the prompt and the edge of the bar
        let area_for_value = self.size.width.saturating_sub(self.prompt.len());

        // we always want to show the left part of the value,
        // therefore the end of the visible range we try to access will be equal to the full width
        let value_end = self.value.width();

        //This should give us the start for the grapheme subrange we want to print out.
        let value_start = value_end.saturating_sub(area_for_value);

        let message = format!(
            "{}{}",
            self.prompt,
            self.value.get_visible_graphemes(value_start..value_end)
        );

        let to_print = if message.len() <= self.size.width {
            message
        } else {
            String::new()
        };

        Terminal::print_row(origin, to_print.as_str())
    }
}
