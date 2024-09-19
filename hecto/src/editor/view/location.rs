use crate::editor::terminal::Position;

#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<Location> for Position {
    fn from(value: Location) -> Self {
        Self {
            col: value.x,
            row: value.y,
        }
    }
}

impl Location {
    pub const fn substract(&self, other: &Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}
