use std::path::Path;
use std::path::PathBuf;


use super::Result;


pub trait PathLike {
    fn to_path(&self) -> Result<&Path>;
    fn as_str(&self) -> Result<&str>;
    fn to_path_buf(&self) -> Result<PathBuf>;
}

impl PathLike for PathBuf {
    fn to_path(&self) -> Result<&Path> {
        Ok(self.as_path())
    }

    fn as_str(&self) -> Result<&str> {
        Ok(self.as_path().to_str().unwrap())
    }

    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(self.clone())
    }
}

impl PathLike for &Path {
    fn to_path(&self) -> Result<&Path> {
        Ok(self)
    }
    
    fn as_str(&self) -> Result<&str> {
        Ok(self.to_str().unwrap())
    }

    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(self))
    }
}

impl PathLike for &str {
    fn to_path(&self) -> Result<&Path> {
        Ok(Path::new(self))
    }
    
    fn as_str(&self) -> Result<&str> {
        Ok(self)
    }

    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(self))
    }
}



