use std::io::Error;

use super::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn load(filename: &str) -> Result<Self, Error> {
        let file_content = std::fs::read_to_string(filename)?;
        let lines = file_content.lines().map(Line::from).collect();
        Ok(Buffer { lines })
    }
}
