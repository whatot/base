use std::fmt::{Display, Formatter};

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone)]
pub enum FileType {
    Rust,
    #[default]
    Text,
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rust => write!(f, "Rust"),
            Self::Text => write!(f, "Text"),
        }
    }
}
