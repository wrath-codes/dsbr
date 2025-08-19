
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DbcError {
    #[error("I/O error at {1}: {0}")]
    IO(io::Error, String),

    #[error("invalid DBC format: {0}")]
    InvalidDbcFormat(String),

    #[error("schema conversion: {0}")]
    SchemaConversion(String),

    #[error("compression: {0}")]
    CompressionError(String),

    #[error("record parsing: {0}")]
    RecordParsingError(String),

    #[error("missing header: {0}")]
    MissingHeader(String),

    #[error("encoding: {0}")]
    Encoding(#[from] crate::models::dbase_utils::DbfEncodingError),

    // (Optionally, if you use Polars here, gate it)
    #[cfg(feature = "polars")]
    #[error("polars: {0}")]
    Polars(#[from] polars::error::PolarsError),
}

impl From<io::Error> for DbcError {
    fn from(e: io::Error) -> Self {
        DbcError::IO(e, "unknown source".to_string())
    }
}

// Keep your helpers:
impl DbcError {
    pub fn io_error<S: AsRef<str>>(err: io::Error, path: S) -> Self {
        Self::IO(err, path.as_ref().to_string())
    }
    pub fn invalid_format<S: Into<String>>(msg: S) -> Self { Self::InvalidDbcFormat(msg.into()) }
    pub fn schema_conversion<S: Into<String>>(msg: S) -> Self { Self::SchemaConversion(msg.into()) }
    pub fn compression_error<S: Into<String>>(msg: S) -> Self { Self::CompressionError(msg.into()) }
    pub fn record_parsing<S: Into<String>>(msg: S) -> Self { Self::RecordParsingError(msg.into()) }
    pub fn missing_header<S: Into<String>>(msg: S) -> Self { Self::MissingHeader(msg.into()) }
}