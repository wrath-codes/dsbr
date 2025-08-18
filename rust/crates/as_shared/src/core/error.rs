use thiserror::Error;

/// A single error type for the whole shared crate.
#[derive(Error, Debug)]
pub enum SharedError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("FTP error: {0}")]
    Ftp(#[from] suppaftp::FtpError),

    #[error("Polars error: {0}")]
    Polars(#[from] polars::error::PolarsError),

    #[error("DBase error: {0}")]
    DBase(#[from] dbase::Error),

    #[error("Invalid filename: {0}")]
    InvalidFilename(String),

    #[error("Other: {0}")]
    Other(String),
}
