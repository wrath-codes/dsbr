use thiserror::Error;
use std::sync::LazyLock;
use dashmap::DashSet;

/// Maximum allowed path length (platform-dependent, using conservative value)
pub const MAX_PATH_LENGTH: usize = 512;

/// Set of invalid characters for paths on Windows (most restrictive)
pub static INVALID_PATH_CHARS: LazyLock<DashSet<char>> = LazyLock::new(|| {
    let chars = DashSet::new();
    
    // Insert specific invalid characters
    ['<', '>', ':', '"', '|', '?', '*']
        .iter()
        .for_each(|&c| { chars.insert(c); });
    
    // Insert control characters (0-31) using functional approach
    (0..32)
        .map(char::from)
        .for_each(|c| { chars.insert(c); });
    
    chars
});

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