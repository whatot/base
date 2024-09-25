use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
};

use crate::editor::FileType;

#[derive(Default, Debug, Clone)]
pub struct FileInfo {
    path: Option<PathBuf>,
    file_type: FileType,
}

impl FileInfo {
    pub fn from(file_name: &str) -> FileInfo {
        let path = PathBuf::from(file_name);
        let file_type = if path
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("rs"))
        {
            FileType::Rust
        } else {
            FileType::Text
        };

        Self {
            path: Some(path),
            file_type,
        }
    }

    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub const fn has_path(&self) -> bool {
        self.path.is_some()
    }

    pub const fn get_file_type(&self) -> FileType {
        self.file_type
    }
}

impl Display for FileInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self
            .get_path()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");
        write!(formatter, "{name}")
    }
}
