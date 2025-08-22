use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum UtilsError {
    #[error("Day error: {0}")]
    Day(#[from] crate::utils::time::day::DayError),

    #[error("Month error: {0}")]
    Month(#[from] crate::utils::time::month::MonthError),

    #[error("Path error: {0}")]
    Path(#[from] crate::utils::path::PathError),

    #[error("Year error: {0}")]
    Year(#[from] crate::utils::time::year::YearError),

    #[error("Duration error: {0}")]
    Duration(#[from] crate::utils::time::duration::DurationError),

    #[error("DateTime error: {0}")]
    DateTime(#[from] crate::utils::time::datetime::DateTimeError),
}
