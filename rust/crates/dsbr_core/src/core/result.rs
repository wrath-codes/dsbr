use super::error::SharedError;

/// A simple alias for `std::result::Result<T, SharedError>`.
pub type Result<T> = std::result::Result<T, SharedError>;
