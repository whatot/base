use super::Location;
use crate::editor::{line::Line, Position};

pub struct SearchInfo {
    pub prev_location: Location,
    pub prev_scroll_offset: Position,
    pub query: Line,
}
