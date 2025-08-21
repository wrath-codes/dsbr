use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use crate::core::Result;
use crate::utils::UtilsError;

pub mod error;
pub mod traits;
#[cfg(test)]
mod tests;

pub use error::{PathError, MAX_PATH_LENGTH, INVALID_PATH_CHARS};
pub use traits::{PathValidatable, PathFromInput, PathLike};

/// A validated path wrapper that ensures the path meets all validation criteria
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValidatedPath {
    pub(crate) inner: PathBuf,
}

impl ValidatedPath {
    /// Create a new ValidatedPath from any path-like input
    pub fn new<T>(input: T) -> Result<Self>
    where
        T: PathFromInput,
    {
        input.parse_path()
    }
    
    /// Get the inner PathBuf
    pub fn into_path_buf(self) -> PathBuf {
        self.inner
    }
    
    /// Get a reference to the inner Path
    pub fn as_path(&self) -> &Path {
        &self.inner
    }
    
    /// Check if the path is absolute
    pub fn is_absolute(&self) -> bool {
        self.inner.is_absolute()
    }
    
    /// Check if the path is relative
    pub fn is_relative(&self) -> bool {
        self.inner.is_relative()
    }
    
    /// Get the file name component
    pub fn file_name(&self) -> Option<&OsStr> {
        self.inner.file_name()
    }
    
    /// Get the parent directory
    pub fn parent(&self) -> Option<&Path> {
        self.inner.parent()
    }
    
    /// Get the file extension
    pub fn extension(&self) -> Option<&OsStr> {
        self.inner.extension()
    }
    
    /// Join with another path component
    pub fn join<P: AsRef<Path>>(&self, path: P) -> Result<ValidatedPath> {
        let joined = self.inner.join(path);
        if !joined.is_valid_path() {
            return Err(UtilsError::Path(PathError::invalid_path(
                format!("Joined path is invalid: {}", joined.display())
            )).into());
        }
        Ok(ValidatedPath { inner: joined })
    }
    
    /// Convert to string representation
    pub fn to_string_lossy(&self) -> std::borrow::Cow<'_, str> {
        self.inner.to_string_lossy()
    }
    
    /// Check if a path is valid without creating a ValidatedPath instance
    pub fn is_valid<T: PathValidatable>(input: T) -> bool {
        input.is_valid_path()
    }
    
    /// Validate a path and return detailed error information
    pub fn validate<T: PathValidatable>(input: T) -> Result<()> {
        if !input.is_valid_path() {
            return Err(UtilsError::Path(PathError::invalid_path(
                "Invalid path provided".to_string()
            )).into());
        }
        Ok(())
    }
}