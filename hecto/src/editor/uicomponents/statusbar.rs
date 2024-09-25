use crate::editor::{
    documentstatus::DocumentStatus, terminal::Terminal, uicomponents::UIComponent,
};
use crate::prelude::*;

#[derive(Default)]
pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    size: Size,
}

impl StatusBar {
    pub fn update_status(&mut self, status: DocumentStatus) {
        if self.current_status != status {
            self.current_status = status;
            self.set_needs_redraw(true);
        }
    }
}

impl UIComponent for StatusBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, origin_y: RowIdx) -> Result<(), std::io::Error> {
        //Assemble the first part of the status bar
        let line_count = self.current_status.line_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();
        let beginning = format!(
            "{} {line_count} {modified_indicator}",
            self.current_status.file_name
        );

        // Assemble the back part
        let position_indicator = self.current_status.position_indicator_to_string();
        let file_type = self.current_status.file_type_to_string();
        let back_part = format!("{file_type} {position_indicator}");

        // Assemble the whole status bar
        let remainder_len = self.size.width.saturating_sub(beginning.len());
        let status = format!("{beginning}{back_part:>remainder_len$}");

        // Only print out the status if it fits. Otherwise write out an empty string to ensure the row is cleared.
        let to_print = if status.len() <= self.size.width {
            status
        } else {
            String::new()
        };

        Terminal::print_inverted_row(origin_y, &to_print)?;

        Ok(())
    }
}
