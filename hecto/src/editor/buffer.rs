use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn load(filename: &str) -> Result<Self, Error> {
        let file_content = std::fs::read_to_string(filename)?;
        let lines = file_content.lines().map(ToString::to_string).collect();
        Ok(Buffer { lines })
    }
}
