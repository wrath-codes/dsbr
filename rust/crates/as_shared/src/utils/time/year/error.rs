use thiserror::Error;

/// Year validation constants
pub const MIN_YEAR: i32 = 1900;
pub const MAX_YEAR: i32 = 2100;
pub const PIVOT_YEAR: i32 = 50;  // 2-digit years 00-49 = 2000-2049, 50-99 = 1950-1999
pub const CURRENT_CENTURY_START: i32 = 2000;
pub const PREVIOUS_CENTURY_START: i32 = 1900;

#[derive(Error, Debug)]
pub enum YearError {
    #[error("Invalid year: {0}. Must be between {1} and {2}")]
    InvalidYear(i32, i32, i32),
    
    #[error("Invalid 2-digit year: {0}. Must be between 00 and 99")]
    Invalid2DigitYear(String),
    
    #[error("Invalid 4-digit year: {0}. Must be between {1} and {2}")]
    Invalid4DigitYear(String, i32, i32),
    
    #[error("Cannot parse year: {0}")]
    CannotParseYear(String),
    
    #[error("Chrono conversion error: {0}")]
    ChronoConversion(String),
    
    #[error("Year arithmetic overflow: {0}")]
    ArithmeticOverflow(String),
    
    #[error("Invalid date for year {0}: {1}")]
    InvalidDate(i32, String),
    
    #[error("Invalid quarter: {0}. Must be between 1 and 4")]
    InvalidQuarter(u8),
    
    #[error("Not a valid year: {0}. This value cannot be converted to a year.")]
    NotValidYear(String),
}

impl YearError {
    pub fn invalid_year(year: i32) -> Self {
        Self::InvalidYear(year, MIN_YEAR, MAX_YEAR)
    }
    
    pub fn invalid_2digit_year<S: Into<String>>(year: S) -> Self {
        Self::Invalid2DigitYear(year.into())
    }
    
    pub fn invalid_4digit_year<S: Into<String>>(year: S) -> Self {
        Self::Invalid4DigitYear(year.into(), MIN_YEAR, MAX_YEAR)
    }
    
    pub fn cannot_parse_year<S: Into<String>>(msg: S) -> Self {
        Self::CannotParseYear(msg.into())
    }
    
    pub fn chrono_conversion<S: Into<String>>(msg: S) -> Self {
        Self::ChronoConversion(msg.into())
    }
    
    pub fn arithmetic_overflow<S: Into<String>>(msg: S) -> Self {
        Self::ArithmeticOverflow(msg.into())
    }
    
    pub fn invalid_date<S: Into<String>>(year: i32, msg: S) -> Self {
        Self::InvalidDate(year, msg.into())
    }
    
    pub fn invalid_quarter(quarter: u8) -> Self {
        Self::InvalidQuarter(quarter)
    }
    
    pub fn not_valid_year<S: Into<String>>(msg: S) -> Self {
        Self::NotValidYear(msg.into())
    }
}