use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::core::{Result, path::PathLike};
    
pub trait Info {
    fn name(&self) -> Result<String>;
    fn path(&self) -> Result<String>;
    fn size(&self) -> Result<u64>;
    fn modified(&self) -> Result<DateTime<Utc>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
}


impl Info for FileInfo {
    fn name(&self) -> Result<String> {
        Ok(self.name.clone())
    }
    fn path(&self) -> Result<String> {
        Ok(self.path.clone())
    }
    fn size(&self) -> Result<u64> {
        Ok(self.size)
    }
    fn modified(&self) -> Result<DateTime<Utc>> {
        Ok(self.modified)
    }
}

impl FileInfo {
    pub fn new(file_path: PathBuf) -> Result<Self> {
        let name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        let path = file_path.to_str().unwrap().to_string();
        let size = file_path.metadata().unwrap().len();
        let modified = file_path.metadata().unwrap().modified().unwrap().into();
        Ok(Self {
            name,
            path,
            size,
            modified,
        })
    }
}
