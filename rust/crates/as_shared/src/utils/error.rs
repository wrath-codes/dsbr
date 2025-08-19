use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum UtilsError {
    #[error("Month error: {0}")]
    Month(#[from] crate::utils::time::MonthError),

    #[error("Path error: {0}")]
    Path(#[from] crate::utils::path::PathError),
}
