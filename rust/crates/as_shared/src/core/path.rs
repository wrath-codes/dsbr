use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;


use super::Result;


pub trait PathLike {
    fn as_path(&self) -> Result<&Path>;
    fn to_path_buf(&self) -> Result<PathBuf>;
    fn as_string(&self) -> Result<String>;
}

impl PathLike for PathBuf {
    fn as_path(&self) -> Result<&Path> {
        Ok(self.as_path())
    }

    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(self.clone())
    }

    fn as_string(&self) -> Result<String> {
        Ok(self.to_string_lossy().to_string())
    }
}

impl PathLike for &Path {
    fn as_path(&self) -> Result<&Path> {
        Ok(self)
    }
    
    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(self))
    }

    fn as_string(&self) -> Result<String> {
        Ok(self.to_string_lossy().to_string())
    }
}


impl PathLike for String {
    fn as_path(&self) -> Result<&Path> {
        Ok(Path::new(self))
    }
    
    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(self))
    }

    fn as_string(&self) -> Result<String> {
        Ok(self.clone().to_string())
    }
}

impl PathLike for &String {
    fn as_path(&self) -> Result<&Path> {
        Ok(Path::new(self))
    }
    
    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(self))
    }

    fn as_string(&self) -> Result<String> {
        Ok(self.clone().to_string())
    }
}

impl PathLike for OsStr {
    fn as_path(&self) -> Result<&Path> {
        Ok(Path::new(self))
    }
    
    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(self))
    }

    fn as_string(&self) -> Result<String> {
        Ok(self.to_string_lossy().to_string())
    }
}

impl PathLike for &OsStr {
    fn as_path(&self) -> Result<&Path> {
        Ok(Path::new(self))
    }
    
    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(self))
    }

    fn as_string(&self) -> Result<String> {
        Ok(self.to_string_lossy().to_string())
    }
}

impl PathLike for OsString {
    fn as_path(&self) -> Result<&Path> {
        Ok(Path::new(self))
    }
    
    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(self))
    }

    fn as_string(&self) -> Result<String> {
        Ok(self.to_string_lossy().to_string())
    }
}

impl PathLike for &OsString {
    fn as_path(&self) -> Result<&Path> {
        Ok(Path::new(self))
    }
    
    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(self))
    }

    fn as_string(&self) -> Result<String> {
        Ok(self.to_string_lossy().to_string())
    }
}

impl PathLike for &str {
    fn as_path(&self) -> Result<&Path> {
        Ok(Path::new(self))
    }
    
    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(self))
    }

    fn as_string(&self) -> Result<String> {
        Ok(self.to_string())
    }
}


