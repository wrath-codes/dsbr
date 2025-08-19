use thiserror::Error;

/// A single error type for the whole shared crate.
#[derive(Error, Debug)]
pub enum SharedError {
    #[error("AWS error: {0}")]
    Aws(#[from] aws_sdk_s3::Error),
    
    #[error("Arrow error: {0}")]
    Arrow(#[from] arrow::error::ArrowError),

    #[error("DBase error: {0}")]
    Dbase(#[from] dbase::Error), 
    
    #[error("Explode error: {0}")]
    Explode(#[from] explode::Error), 

    #[error("FTP error: {0}")]
    Ftp(#[from] suppaftp::FtpError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::error::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Polars error: {0}")]
    Polars(#[from] polars::error::PolarsError),
    
    #[error("Rayon error: {0}")]
    Rayon(#[from] rayon::ThreadPoolBuildError),
    
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    
    #[error("Utils error: {0}")]
    Utils(#[from] crate::utils::UtilsError),

    #[error("Path error: {0}")]
    Path(#[from] crate::core::path::PathError),

    #[error("Invalid filename: {0}")]
    InvalidFilename(String),
    
    #[error("Other: {0}")]
    Other(String),
}
