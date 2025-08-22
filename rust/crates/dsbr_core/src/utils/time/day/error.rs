use thiserror::Error;

#[derive(Error, Debug)]
pub enum DayError {
    #[error("Invalid day: {0}. Must be between 1 and 31")]
    InvalidDay(u8),
    
    #[error("Invalid day for month: day {0} does not exist in {1}")]
    InvalidDayForMonth(u8, String),
    
    #[error("Cannot parse day: {0}")]
    CannotParseDay(String),
    
    #[error("Not a valid day: {0}. This value cannot be converted to a day.")]
    NotValidDay(String),
    
    #[error("Day arithmetic overflow: {0}")]
    ArithmeticOverflow(String),
    
    #[error("Chrono conversion error: {0}")]
    ChronoConversion(String),
}

impl DayError {
    pub fn invalid_day(day: u8) -> Self {
        Self::InvalidDay(day)
    }
    
    pub fn invalid_day_for_month<S: Into<String>>(day: u8, month: S) -> Self {
        Self::InvalidDayForMonth(day, month.into())
    }
    
    pub fn cannot_parse_day<S: Into<String>>(msg: S) -> Self {
        Self::CannotParseDay(msg.into())
    }
    
    pub fn not_valid_day<S: Into<String>>(msg: S) -> Self {
        Self::NotValidDay(msg.into())
    }
    
    pub fn arithmetic_overflow<S: Into<String>>(msg: S) -> Self {
        Self::ArithmeticOverflow(msg.into())
    }
    
    pub fn chrono_conversion<S: Into<String>>(msg: S) -> Self {
        Self::ChronoConversion(msg.into())
    }
}