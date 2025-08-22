use thiserror::Error;

#[derive(Error, Debug)]
pub enum DateTimeError {
    #[error("Invalid datetime: {0}")]
    InvalidDateTime(String),
    
    #[error("Invalid time component: {0}")]
    InvalidTimeComponent(String),
    
    #[error("Invalid date component: {0}")]
    InvalidDateComponent(String),
    
    #[error("Cannot parse datetime: {0}")]
    CannotParseDateTime(String),
    
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    
    #[error("Timestamp conversion error: {0}")]
    TimestampConversion(String),
    
    #[error("Chrono conversion error: {0}")]
    ChronoConversion(String),
    
    #[error("Arithmetic overflow: {0}")]
    ArithmeticOverflow(String),
    
    #[error("Arithmetic underflow: {0}")]
    ArithmeticUnderflow(String),
    
    #[error("Invalid timezone: {0}")]
    InvalidTimezone(String),
}

impl DateTimeError {
    pub fn invalid_datetime<S: Into<String>>(msg: S) -> Self {
        Self::InvalidDateTime(msg.into())
    }
    
    pub fn invalid_time_component<S: Into<String>>(msg: S) -> Self {
        Self::InvalidTimeComponent(msg.into())
    }
    
    pub fn invalid_date_component<S: Into<String>>(msg: S) -> Self {
        Self::InvalidDateComponent(msg.into())
    }
    
    pub fn cannot_parse_datetime<S: Into<String>>(msg: S) -> Self {
        Self::CannotParseDateTime(msg.into())
    }
    
    pub fn invalid_format<S: Into<String>>(msg: S) -> Self {
        Self::InvalidFormat(msg.into())
    }
    
    pub fn timestamp_conversion<S: Into<String>>(msg: S) -> Self {
        Self::TimestampConversion(msg.into())
    }
    
    pub fn chrono_conversion<S: Into<String>>(msg: S) -> Self {
        Self::ChronoConversion(msg.into())
    }
    
    pub fn arithmetic_overflow<S: Into<String>>(msg: S) -> Self {
        Self::ArithmeticOverflow(msg.into())
    }
    
    pub fn arithmetic_underflow<S: Into<String>>(msg: S) -> Self {
        Self::ArithmeticUnderflow(msg.into())
    }
    
    pub fn invalid_timezone<S: Into<String>>(msg: S) -> Self {
        Self::InvalidTimezone(msg.into())
    }
}