use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use dashmap::DashSet;
use serde::{Serialize, Deserialize};
use thiserror::Error;

use super::Result;

/// Errors that can occur when working with paths
#[derive(Error, Debug)]
pub enum PathError {
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    #[error("Path contains invalid UTF-8: {0}")]
    InvalidUtf8(String),
    
    #[error("Path is empty")]
    EmptyPath,
    
    #[error("Path is too long: {0} characters (max allowed: {1})")]
    PathTooLong(usize, usize),
    
    #[error("Path contains invalid characters: {0}")]
    InvalidCharacters(String),
    
    #[error("Cannot convert path: {0}")]
    CannotConvertPath(String),
    
    #[error("Cannot parse path: {0}")]
    CannotParsePath(String),
    
    #[error("Path does not exist: {0}")]
    PathNotFound(String),
    
    #[error("Path is not absolute: {0}")]
    NotAbsolute(String),
    
    #[error("Path is not relative: {0}")]
    NotRelative(String),
}

impl PathError {
    pub fn invalid_path<S: Into<String>>(msg: S) -> Self {
        Self::InvalidPath(msg.into())
    }
    
    pub fn invalid_utf8<S: Into<String>>(msg: S) -> Self {
        Self::InvalidUtf8(msg.into())
    }
    
    pub fn empty_path() -> Self {
        Self::EmptyPath
    }
    
    pub fn path_too_long(actual: usize, max: usize) -> Self {
        Self::PathTooLong(actual, max)
    }
    
    pub fn invalid_characters<S: Into<String>>(msg: S) -> Self {
        Self::InvalidCharacters(msg.into())
    }
    
    pub fn cannot_convert_path<S: Into<String>>(msg: S) -> Self {
        Self::CannotConvertPath(msg.into())
    }
    
    pub fn cannot_parse_path<S: Into<String>>(msg: S) -> Self {
        Self::CannotParsePath(msg.into())
    }
    
    pub fn path_not_found<S: Into<String>>(msg: S) -> Self {
        Self::PathNotFound(msg.into())
    }
    
    pub fn not_absolute<S: Into<String>>(msg: S) -> Self {
        Self::NotAbsolute(msg.into())
    }
    
    pub fn not_relative<S: Into<String>>(msg: S) -> Self {
        Self::NotRelative(msg.into())
    }
}

/// Maximum allowed path length (platform-dependent, using conservative value)
pub const MAX_PATH_LENGTH: usize = 512;

/// Set of invalid characters for paths on Windows (most restrictive)
pub static INVALID_PATH_CHARS: LazyLock<DashSet<char>> = LazyLock::new(|| {
    let chars = DashSet::new();
    chars.insert('<');
    chars.insert('>');
    chars.insert(':');
    chars.insert('"');
    chars.insert('|');
    chars.insert('?');
    chars.insert('*');
    // Control characters (0-31)
    for c in 0..32 {
        chars.insert(char::from(c));
    }
    chars
});

/// Trait for types that can be validated as paths
///
/// # Examples
///
/// ```
/// use arrow_sus_shared::core::path::PathValidatable;
/// use std::path::PathBuf;
///
/// // String validation
/// assert!("/valid/path".is_valid_path());
/// assert!(!"".is_valid_path()); // Empty path
/// assert!(!"path/with\0null".is_valid_path()); // Contains null character
///
/// // PathBuf validation
/// let path = PathBuf::from("/valid/path");
/// assert!(path.is_valid_path());
///
/// // Character validation
/// assert!("valid/path/file.txt".has_valid_characters());
/// assert!(!"path/with<bracket".has_valid_characters());
///
/// // Length validation
/// let long_path = "a".repeat(513);
/// assert!(!long_path.has_valid_length());
/// ```
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
///
/// # Examples
///
/// ```
/// use arrow_sus_shared::core::path::{PathFromInput, ValidatedPath};
/// use std::path::PathBuf;
///
/// // String parsing
/// let path = "/valid/path".parse_path().unwrap();
/// assert_eq!(path.as_path().to_str().unwrap(), "/valid/path");
///
/// // PathBuf parsing
/// let pathbuf = PathBuf::from("/another/path");
/// let path = pathbuf.parse_path().unwrap();
/// assert_eq!(path.into_path_buf(), PathBuf::from("/another/path"));
///
/// // Invalid cases
/// assert!("".parse_path().is_err()); // Empty path
/// assert!("path/with\0null".parse_path().is_err()); // Invalid characters
/// ```
pub trait PathFromInput {
    fn parse_path(self) -> Result<ValidatedPath>;
}

/// A validated path wrapper that ensures the path meets all validation criteria
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValidatedPath {
    inner: PathBuf,
}

impl ValidatedPath {
    /// Create a new ValidatedPath from any path-like input
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::core::path::ValidatedPath;
    /// use std::path::PathBuf;
    ///
    /// // From string
    /// let path = ValidatedPath::new("/valid/path").unwrap();
    /// assert_eq!(path.to_string_lossy(), "/valid/path");
    ///
    /// // From PathBuf
    /// let pathbuf = PathBuf::from("/another/path");
    /// let path = ValidatedPath::new(pathbuf).unwrap();
    /// assert_eq!(path.to_string_lossy(), "/another/path");
    ///
    /// // Invalid cases
    /// assert!(ValidatedPath::new("").is_err()); // Empty path
    /// assert!(ValidatedPath::new("path/with\0null").is_err()); // Invalid characters
    /// ```
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
            return Err(PathError::invalid_path(
                format!("Joined path is invalid: {}", joined.display())
            ).into());
        }
        Ok(ValidatedPath { inner: joined })
    }
    
    /// Convert to string representation
    pub fn to_string_lossy(&self) -> std::borrow::Cow<'_, str> {
        self.inner.to_string_lossy()
    }
}

impl PathFromInput for &str {
    fn parse_path(self) -> Result<ValidatedPath> {
        if !self.is_valid_path() {
            if self.is_empty() {
                return Err(PathError::empty_path().into());
            }
            if !self.has_valid_length() {
                return Err(PathError::path_too_long(self.len(), MAX_PATH_LENGTH).into());
            }
            if !self.has_valid_characters() {
                return Err(PathError::invalid_characters(
                    format!("Path contains invalid characters: {}", self)
                ).into());
            }
            return Err(PathError::invalid_path(self.to_string()).into());
        }
        
        Ok(ValidatedPath {
            inner: PathBuf::from(self)
        })
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
        if !self.is_valid_path() {
            let path_str = self.to_string_lossy();
            if path_str.is_empty() {
                return Err(PathError::empty_path().into());
            }
            if !self.has_valid_length() {
                return Err(PathError::path_too_long(
                    self.as_os_str().len(), 
                    MAX_PATH_LENGTH
                ).into());
            }
            if !self.has_valid_characters() {
                return Err(PathError::invalid_characters(
                    format!("Path contains invalid characters: {}", path_str)
                ).into());
            }
            return Err(PathError::invalid_path(path_str.to_string()).into());
        }
        
        Ok(ValidatedPath { inner: self })
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
///
/// # Examples
///
/// ```
/// use arrow_sus_shared::core::path::PathLike;
/// use std::path::PathBuf;
///
/// // Working with PathBuf
/// let pathbuf = PathBuf::from("/valid/path");
/// assert_eq!(pathbuf.as_string().unwrap(), "/valid/path");
/// assert!(pathbuf.validate().is_ok());
///
/// // Working with &str
/// let path_str = "/another/path";
/// assert_eq!(path_str.as_string().unwrap(), "/another/path");
/// assert!(path_str.validate().is_ok());
///
/// // Working with String
/// let path_string = String::from("/string/path");
/// assert_eq!(path_string.as_string().unwrap(), "/string/path");
/// assert!(path_string.validate().is_ok());
/// ```
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
            .ok_or_else(|| PathError::invalid_utf8(
                format!("Path contains invalid UTF-8: {}", self.display())
            ).into())
    }
    
    fn validate(&self) -> Result<()> {
        if !self.is_valid_path() {
            return Err(PathError::invalid_path(
                format!("Invalid path: {}", self.display())
            ).into());
        }
        Ok(())
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
            .map_err(|e| PathError::cannot_convert_path(
                format!("Cannot canonicalize path {}: {}", self.display(), e)
            ).into())
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
            .ok_or_else(|| PathError::invalid_utf8(
                format!("Path contains invalid UTF-8: {}", self.display())
            ).into())
    }
    
    fn validate(&self) -> Result<()> {
        if !self.is_valid_path() {
            return Err(PathError::invalid_path(
                format!("Invalid path: {}", self.display())
            ).into());
        }
        Ok(())
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
            .map_err(|e| PathError::cannot_convert_path(
                format!("Cannot canonicalize path {}: {}", self.display(), e)
            ).into())
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
        if !self.is_valid_path() {
            return Err(PathError::invalid_path(self.clone()).into());
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
            .map_err(|e| PathError::cannot_convert_path(
                format!("Cannot canonicalize path {}: {}", self, e)
            ).into())
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
        if !self.is_valid_path() {
            return Err(PathError::invalid_path((*self).clone()).into());
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
            .map_err(|e| PathError::cannot_convert_path(
                format!("Cannot canonicalize path {}: {}", self, e)
            ).into())
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
            .ok_or_else(|| PathError::invalid_utf8(
                format!("OsStr contains invalid UTF-8: {:?}", self)
            ).into())
    }
    
    fn validate(&self) -> Result<()> {
        if !self.is_valid_path() {
            return Err(PathError::invalid_path(
                format!("Invalid path: {:?}", self)
            ).into());
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
            .map_err(|e| PathError::cannot_convert_path(
                format!("Cannot canonicalize path {:?}: {}", self, e)
            ).into())
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
            .ok_or_else(|| PathError::invalid_utf8(
                format!("OsStr contains invalid UTF-8: {:?}", self)
            ).into())
    }
    
    fn validate(&self) -> Result<()> {
        if !self.is_valid_path() {
            return Err(PathError::invalid_path(
                format!("Invalid path: {:?}", self)
            ).into());
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
            .map_err(|e| PathError::cannot_convert_path(
                format!("Cannot canonicalize path {:?}: {}", self, e)
            ).into())
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
            .ok_or_else(|| PathError::invalid_utf8(
                format!("OsString contains invalid UTF-8: {:?}", self)
            ).into())
    }
    
    fn validate(&self) -> Result<()> {
        if !self.is_valid_path() {
            return Err(PathError::invalid_path(
                format!("Invalid path: {:?}", self)
            ).into());
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
            .map_err(|e| PathError::cannot_convert_path(
                format!("Cannot canonicalize path {:?}: {}", self, e)
            ).into())
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
            .ok_or_else(|| PathError::invalid_utf8(
                format!("OsString contains invalid UTF-8: {:?}", self)
            ).into())
    }
    
    fn validate(&self) -> Result<()> {
        if !self.is_valid_path() {
            return Err(PathError::invalid_path(
                format!("Invalid path: {:?}", self)
            ).into());
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
            .map_err(|e| PathError::cannot_convert_path(
                format!("Cannot canonicalize path {:?}: {}", self, e)
            ).into())
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
            return Err(PathError::invalid_path(self.to_string()).into());
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
            .map_err(|e| PathError::cannot_convert_path(
                format!("Cannot canonicalize path {}: {}", self, e)
            ).into())
    }
}

/// Utility functions for path operations
impl ValidatedPath {
    /// Check if a path is valid without creating a ValidatedPath instance
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::core::path::ValidatedPath;
    /// use std::path::PathBuf;
    ///
    /// assert!(ValidatedPath::is_valid("/valid/path"));
    /// assert!(!ValidatedPath::is_valid(""));
    /// assert!(!ValidatedPath::is_valid("path/with\0null"));
    ///
    /// // Works with different types
    /// assert!(ValidatedPath::is_valid(String::from("/string/path")));
    /// assert!(ValidatedPath::is_valid(PathBuf::from("/pathbuf/path")));
    /// ```
    pub fn is_valid<T: PathValidatable>(input: T) -> bool {
        input.is_valid_path()
    }
    
    /// Validate a path and return detailed error information
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::core::path::ValidatedPath;
    ///
    /// assert!(ValidatedPath::validate("/valid/path").is_ok());
    /// assert!(ValidatedPath::validate("").is_err());
    /// assert!(ValidatedPath::validate("path/with\0null").is_err());
    ///
    /// // Test path length validation
    /// let long_path = "a".repeat(513);
    /// assert!(ValidatedPath::validate(long_path).is_err());
    /// ```
    pub fn validate<T: PathValidatable>(input: T) -> Result<()> {
        if !input.is_valid_path() {
            return Err(PathError::invalid_path(
                "Invalid path provided".to_string()
            ).into());
        }
        Ok(())
    }
}

