use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::info::FileInfo;
use std::fmt;

use crate::core::Result;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub extension: String,
    pub basename: String,
    pub path: String,
    pub info: FileInfo,
}

impl File {
    pub fn new(file_path: String) -> Self {
        let full_path = file_path.to_path().unwrap();
        let name = full_path.file_name().unwrap().to_str().unwrap().to_string();
        let extension = full_path.extension().unwrap().to_str().unwrap().to_string();
        let basename = full_path.file_stem().unwrap().to_str().unwrap().to_string();
        let path = file_path.clone();
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

    