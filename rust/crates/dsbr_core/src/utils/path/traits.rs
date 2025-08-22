use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use crate::core::Result;
use crate::utils::UtilsError;
use super::{ValidatedPath, PathError};
use super::error::{MAX_PATH_LENGTH, INVALID_PATH_CHARS};

/// Trait for types that can be validated as paths
pub trait PathValidatable {
    fn is_valid_path(&self) -> bool;
    fn is_absolute_path(&self) -> bool;
    fn is_relative_path(&self) -> bool;
    fn has_valid_length(&self) -> bool;
    fn has_valid_characters(&self) -> bool;
}

impl PathValidatable for str {
    fn is_valid_path(&self) -> bool {
        !self.is_empty() && 
        self.has_valid_length() && 
        self.has_valid_characters()
    }
    
    fn is_absolute_path(&self) -> bool {
        Path::new(self).is_absolute()
    }
    
    fn is_relative_path(&self) -> bool {
        Path::new(self).is_relative()
    }
    
    fn has_valid_length(&self) -> bool {
        self.len() <= MAX_PATH_LENGTH
    }
    
    fn has_valid_characters(&self) -> bool {
        !self.chars().any(|c| INVALID_PATH_CHARS.contains(&c))
    }
}

impl PathValidatable for &str {
    fn is_valid_path(&self) -> bool {
        (*self).is_valid_path()
    }
    
    fn is_absolute_path(&self) -> bool {
        (*self).is_absolute_path()
    }
    
    fn is_relative_path(&self) -> bool {
        (*self).is_relative_path()
    }
    
    fn has_valid_length(&self) -> bool {
        (*self).has_valid_length()
    }
    
    fn has_valid_characters(&self) -> bool {
        (*self).has_valid_characters()
    }
}

impl PathValidatable for String {
    fn is_valid_path(&self) -> bool {
        self.as_str().is_valid_path()
    }
    
    fn is_absolute_path(&self) -> bool {
        self.as_str().is_absolute_path()
    }
    
    fn is_relative_path(&self) -> bool {
        self.as_str().is_relative_path()
    }
    
    fn has_valid_length(&self) -> bool {
        self.as_str().has_valid_length()
    }
    
    fn has_valid_characters(&self) -> bool {
        self.as_str().has_valid_characters()
    }
}

impl PathValidatable for &String {
    fn is_valid_path(&self) -> bool {
        self.as_str().is_valid_path()
    }
    
    fn is_absolute_path(&self) -> bool {
        self.as_str().is_absolute_path()
    }
    
    fn is_relative_path(&self) -> bool {
        self.as_str().is_relative_path()
    }
    
    fn has_valid_length(&self) -> bool {
        self.as_str().has_valid_length()
    }
    
    fn has_valid_characters(&self) -> bool {
        self.as_str().has_valid_characters()
    }
}

impl PathValidatable for Path {
    fn is_valid_path(&self) -> bool {
        self.as_os_str().to_string_lossy().is_valid_path()
    }
    
    fn is_absolute_path(&self) -> bool {
        self.is_absolute()
    }
    
    fn is_relative_path(&self) -> bool {
        self.is_relative()
    }
    
    fn has_valid_length(&self) -> bool {
        self.as_os_str().len() <= MAX_PATH_LENGTH
    }
    
    fn has_valid_characters(&self) -> bool {
        self.as_os_str().to_string_lossy().has_valid_characters()
    }
}

impl PathValidatable for &Path {
    fn is_valid_path(&self) -> bool {
        (*self).is_valid_path()
    }
    
    fn is_absolute_path(&self) -> bool {
        (*self).is_absolute_path()
    }
    
    fn is_relative_path(&self) -> bool {
        (*self).is_relative_path()
    }
    
    fn has_valid_length(&self) -> bool {
        (*self).has_valid_length()
    }
    
    fn has_valid_characters(&self) -> bool {
        (*self).has_valid_characters()
    }
}

impl PathValidatable for PathBuf {
    fn is_valid_path(&self) -> bool {
        self.as_path().is_valid_path()
    }
    
    fn is_absolute_path(&self) -> bool {
        self.as_path().is_absolute_path()
    }
    
    fn is_relative_path(&self) -> bool {
        self.as_path().is_relative_path()
    }
    
    fn has_valid_length(&self) -> bool {
        self.as_path().has_valid_length()
    }
    
    fn has_valid_characters(&self) -> bool {
        self.as_path().has_valid_characters()
    }
}

impl PathValidatable for OsStr {
    fn is_valid_path(&self) -> bool {
        self.to_string_lossy().is_valid_path()
    }
    
    fn is_absolute_path(&self) -> bool {
        Path::new(self).is_absolute()
    }
    
    fn is_relative_path(&self) -> bool {
        Path::new(self).is_relative()
    }
    
    fn has_valid_length(&self) -> bool {
        self.len() <= MAX_PATH_LENGTH
    }
    
    fn has_valid_characters(&self) -> bool {
        self.to_string_lossy().has_valid_characters()
    }
}

impl PathValidatable for &OsStr {
    fn is_valid_path(&self) -> bool {
        (*self).is_valid_path()
    }
    
    fn is_absolute_path(&self) -> bool {
        (*self).is_absolute_path()
    }
    
    fn is_relative_path(&self) -> bool {
        (*self).is_relative_path()
    }
    
    fn has_valid_length(&self) -> bool {
        (*self).has_valid_length()
    }
    
    fn has_valid_characters(&self) -> bool {
        (*self).has_valid_characters()
    }
}

impl PathValidatable for OsString {
    fn is_valid_path(&self) -> bool {
        self.as_os_str().is_valid_path()
    }
    
    fn is_absolute_path(&self) -> bool {
        self.as_os_str().is_absolute_path()
    }
    
    fn is_relative_path(&self) -> bool {
        self.as_os_str().is_relative_path()
    }
    
    fn has_valid_length(&self) -> bool {
        self.as_os_str().has_valid_length()
    }
    
    fn has_valid_characters(&self) -> bool {
        self.as_os_str().has_valid_characters()
    }
}

impl PathValidatable for &OsString {
    fn is_valid_path(&self) -> bool {
        self.as_os_str().is_valid_path()
    }
    
    fn is_absolute_path(&self) -> bool {
        self.as_os_str().is_absolute_path()
    }
    
    fn is_relative_path(&self) -> bool {
        self.as_os_str().is_relative_path()
    }
    
    fn has_valid_length(&self) -> bool {
        self.as_os_str().has_valid_length()
    }
    
    fn has_valid_characters(&self) -> bool {
        self.as_os_str().has_valid_characters()
    }
}

/// Trait for types that can be parsed into PathLike using the generic from() method
pub trait PathFromInput {
    fn parse_path(self) -> Result<ValidatedPath>;
}

impl PathFromInput for &str {
    fn parse_path(self) -> Result<ValidatedPath> {
        match self.is_valid_path() {
            true => Ok(ValidatedPath {
                inner: PathBuf::from(self)
            }),
            false => match (self.is_empty(), self.has_valid_length(), self.has_valid_characters()) {
                (true, _, _) => Err(UtilsError::Path(PathError::empty_path()).into()),
                (_, false, _) => Err(UtilsError::Path(PathError::path_too_long(self.len(), MAX_PATH_LENGTH)).into()),
                (_, _, false) => Err(UtilsError::Path(PathError::invalid_characters(
                    format!("Path contains invalid characters: {}", self)
                )).into()),
                _ => Err(UtilsError::Path(PathError::invalid_path(self.to_string())).into()),
            }
        }
    }
}

impl PathFromInput for String {
    fn parse_path(self) -> Result<ValidatedPath> {
        self.as_str().parse_path()
    }
}

impl PathFromInput for &String {
    fn parse_path(self) -> Result<ValidatedPath> {
        self.as_str().parse_path()
    }
}

impl PathFromInput for PathBuf {
    fn parse_path(self) -> Result<ValidatedPath> {
        match self.is_valid_path() {
            true => Ok(ValidatedPath { inner: self }),
            false => {
                let path_str = self.to_string_lossy();
                match (path_str.is_empty(), self.has_valid_length(), self.has_valid_characters()) {
                    (true, _, _) => Err(UtilsError::Path(PathError::empty_path()).into()),
                    (_, false, _) => Err(UtilsError::Path(PathError::path_too_long(
                        self.as_os_str().len(),
                        MAX_PATH_LENGTH
                    )).into()),
                    (_, _, false) => Err(UtilsError::Path(PathError::invalid_characters(
                        format!("Path contains invalid characters: {}", path_str)
                    )).into()),
                    _ => Err(UtilsError::Path(PathError::invalid_path(path_str.to_string())).into()),
                }
            }
        }
    }
}

impl PathFromInput for &Path {
    fn parse_path(self) -> Result<ValidatedPath> {
        self.to_path_buf().parse_path()
    }
}

impl PathFromInput for &PathBuf {
    fn parse_path(self) -> Result<ValidatedPath> {
        self.clone().parse_path()
    }
}

impl PathFromInput for OsString {
    fn parse_path(self) -> Result<ValidatedPath> {
        PathBuf::from(self).parse_path()
    }
}

impl PathFromInput for &OsString {
    fn parse_path(self) -> Result<ValidatedPath> {
        PathBuf::from(self).parse_path()
    }
}

impl PathFromInput for &OsStr {
    fn parse_path(self) -> Result<ValidatedPath> {
        PathBuf::from(self).parse_path()
    }
}

/// Enhanced PathLike trait with comprehensive path operations
///
/// This trait provides a unified interface for working with different path types,
/// with built-in validation and error handling.
pub trait PathLike {
    /// Get a reference to the path as &Path
    fn as_path(&self) -> Result<&Path>;
    
    /// Convert to PathBuf
    fn to_path_buf(&self) -> Result<PathBuf>;
    
    /// Convert to String representation
    fn as_string(&self) -> Result<String>;
    
    /// Validate the path
    fn validate(&self) -> Result<()>;
    
    /// Check if the path exists on the filesystem
    fn exists(&self) -> bool;
    
    /// Check if the path is a file
    fn is_file(&self) -> bool;
    
    /// Check if the path is a directory
    fn is_dir(&self) -> bool;
    
    /// Get the canonical (absolute) path
    fn canonicalize(&self) -> Result<PathBuf>;
}

impl PathLike for PathBuf {
    fn as_path(&self) -> Result<&Path> {
        Ok(self.as_path())
    }

    fn to_path_buf(&self) -> Result<PathBuf> {
        Ok(self.clone())
    }

    fn as_string(&self) -> Result<String> {
        self.to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| UtilsError::Path(PathError::invalid_utf8(
                format!("Path contains invalid UTF-8: {}", self.display())
            )).into())
    }
    
    fn validate(&self) -> Result<()> {
        match self.is_valid_path() {
            true => Ok(()),
            false => Err(UtilsError::Path(PathError::invalid_path(
                format!("Invalid path: {}", self.display())
            )).into()),
        }
    }
    
    fn exists(&self) -> bool {
        Path::exists(self.as_path())
    }
    
    fn is_file(&self) -> bool {
        Path::is_file(self.as_path())
    }
    
    fn is_dir(&self) -> bool {
        Path::is_dir(self.as_path())
    }
    
    fn canonicalize(&self) -> Result<PathBuf> {
        Path::canonicalize(self.as_path())
            .map_err(|e| UtilsError::Path(PathError::cannot_convert_path(
                format!("Cannot canonicalize path {}: {}", self.display(), e)
            )).into())
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
        self.to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| UtilsError::Path(PathError::invalid_utf8(
                format!("Path contains invalid UTF-8: {}", self.display())
            )).into())
    }
    
    fn validate(&self) -> Result<()> {
        match self.is_valid_path() {
            true => Ok(()),
            false => Err(UtilsError::Path(PathError::invalid_path(
                format!("Invalid path: {}", self.display())
            )).into()),
        }
    }
    
    fn exists(&self) -> bool {
        Path::exists(*self)
    }
    
    fn is_file(&self) -> bool {
        Path::is_file(*self)
    }
    
    fn is_dir(&self) -> bool {
        Path::is_dir(*self)
    }
    
    fn canonicalize(&self) -> Result<PathBuf> {
        Path::canonicalize(*self)
            .map_err(|e| UtilsError::Path(PathError::cannot_convert_path(
                format!("Cannot canonicalize path {}: {}", self.display(), e)
            )).into())
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
        Ok(self.clone())
    }
    
    fn validate(&self) -> Result<()> {
        match self.is_valid_path() {
            true => Ok(()),
            false => Err(UtilsError::Path(PathError::invalid_path(self.clone())).into()),
        }
    }
    
    fn exists(&self) -> bool {
        Path::new(self).exists()
    }
    
    fn is_file(&self) -> bool {
        Path::new(self).is_file()
    }
    
    fn is_dir(&self) -> bool {
        Path::new(self).is_dir()
    }
    
    fn canonicalize(&self) -> Result<PathBuf> {
        Path::new(self).canonicalize()
            .map_err(|e| UtilsError::Path(PathError::cannot_convert_path(
                format!("Cannot canonicalize path {}: {}", self, e)
            )).into())
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
        Ok((*self).clone())
    }
    
    fn validate(&self) -> Result<()> {
        match self.is_valid_path() {
            true => Ok(()),
            false => Err(UtilsError::Path(PathError::invalid_path((*self).clone())).into()),
        }
    }
    
    fn exists(&self) -> bool {
        Path::new(self).exists()
    }
    
    fn is_file(&self) -> bool {
        Path::new(self).is_file()
    }
    
    fn is_dir(&self) -> bool {
        Path::new(self).is_dir()
    }
    
    fn canonicalize(&self) -> Result<PathBuf> {
        Path::new(self).canonicalize()
            .map_err(|e| UtilsError::Path(PathError::cannot_convert_path(
                format!("Cannot canonicalize path {}: {}", self, e)
            )).into())
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
        self.to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| UtilsError::Path(PathError::invalid_utf8(
                format!("OsStr contains invalid UTF-8: {:?}", self)
            )).into())
    }
    
    fn validate(&self) -> Result<()> {
        if !self.is_valid_path() {
            return Err(UtilsError::Path(PathError::invalid_path(
                format!("Invalid path: {:?}", self)
            )).into());
        }
        Ok(())
    }
    
    fn exists(&self) -> bool {
        Path::new(self).exists()
    }
    
    fn is_file(&self) -> bool {
        Path::new(self).is_file()
    }
    
    fn is_dir(&self) -> bool {
        Path::new(self).is_dir()
    }
    
    fn canonicalize(&self) -> Result<PathBuf> {
        Path::new(self).canonicalize()
            .map_err(|e| UtilsError::Path(PathError::cannot_convert_path(
                format!("Cannot canonicalize path {:?}: {}", self, e)
            )).into())
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
        self.to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| UtilsError::Path(PathError::invalid_utf8(
                format!("OsStr contains invalid UTF-8: {:?}", self)
            )).into())
    }
    
    fn validate(&self) -> Result<()> {
        if !self.is_valid_path() {
            return Err(UtilsError::Path(PathError::invalid_path(
                format!("Invalid path: {:?}", self)
            )).into());
        }
        Ok(())
    }
    
    fn exists(&self) -> bool {
        Path::new(self).exists()
    }
    
    fn is_file(&self) -> bool {
        Path::new(self).is_file()
    }
    
    fn is_dir(&self) -> bool {
        Path::new(self).is_dir()
    }
    
    fn canonicalize(&self) -> Result<PathBuf> {
        Path::new(self).canonicalize()
            .map_err(|e| UtilsError::Path(PathError::cannot_convert_path(
                format!("Cannot canonicalize path {:?}: {}", self, e)
            )).into())
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
        self.to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| UtilsError::Path(PathError::invalid_utf8(
                format!("OsString contains invalid UTF-8: {:?}", self)
            )).into())
    }
    
    fn validate(&self) -> Result<()> {
        if !self.is_valid_path() {
            return Err(UtilsError::Path(PathError::invalid_path(
                format!("Invalid path: {:?}", self)
            )).into());
        }
        Ok(())
    }
    
    fn exists(&self) -> bool {
        Path::new(self).exists()
    }
    
    fn is_file(&self) -> bool {
        Path::new(self).is_file()
    }
    
    fn is_dir(&self) -> bool {
        Path::new(self).is_dir()
    }
    
    fn canonicalize(&self) -> Result<PathBuf> {
        Path::new(self).canonicalize()
            .map_err(|e| UtilsError::Path(PathError::cannot_convert_path(
                format!("Cannot canonicalize path {:?}: {}", self, e)
            )).into())
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
        self.to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| UtilsError::Path(PathError::invalid_utf8(
                format!("OsString contains invalid UTF-8: {:?}", self)
            )).into())
    }
    
    fn validate(&self) -> Result<()> {
        if !self.is_valid_path() {
            return Err(UtilsError::Path(PathError::invalid_path(
                format!("Invalid path: {:?}", self)
            )).into());
        }
        Ok(())
    }
    
    fn exists(&self) -> bool {
        Path::new(self).exists()
    }
    
    fn is_file(&self) -> bool {
        Path::new(self).is_file()
    }
    
    fn is_dir(&self) -> bool {
        Path::new(self).is_dir()
    }
    
    fn canonicalize(&self) -> Result<PathBuf> {
        Path::new(self).canonicalize()
            .map_err(|e| UtilsError::Path(PathError::cannot_convert_path(
                format!("Cannot canonicalize path {:?}: {}", self, e)
            )).into())
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
    
    fn validate(&self) -> Result<()> {
        if !self.is_valid_path() {
            return Err(UtilsError::Path(PathError::invalid_path(self.to_string())).into());
        }
        Ok(())
    }
    
    fn exists(&self) -> bool {
        Path::new(self).exists()
    }
    
    fn is_file(&self) -> bool {
        Path::new(self).is_file()
    }
    
    fn is_dir(&self) -> bool {
        Path::new(self).is_dir()
    }
    
    fn canonicalize(&self) -> Result<PathBuf> {
        Path::new(self).canonicalize()
            .map_err(|e| UtilsError::Path(PathError::cannot_convert_path(
                format!("Cannot canonicalize path {}: {}", self, e)
            )).into())
    }
}