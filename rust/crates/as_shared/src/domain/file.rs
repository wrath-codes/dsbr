use serde::{Deserialize, Serialize};
use super::info::FileInfo;
use std::fmt;

use crate::core::{path::PathLike};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub extension: String,
    pub basename: String,
    pub path: String,
    pub info: FileInfo,
}

impl File {
    pub fn new(file_path: Box<dyn PathLike>) -> Self {
        let full_path = file_path.as_path().unwrap();
        let name = full_path.file_name().unwrap().to_string_lossy().to_string();
        let extension = full_path.extension().unwrap().to_string_lossy().to_string();
        let basename = full_path.file_stem().unwrap().to_string_lossy().to_string();
        let path = file_path.as_string().unwrap();
        let info = FileInfo::new(full_path.to_path_buf()).unwrap();

        Self {
            name,
            extension,
            basename,
            path,
            info,
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}
