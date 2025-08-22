use thiserror::Error;

#[derive(Error, Debug)]
pub enum DurationError {
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),
    
    #[error("Duration overflow: {0}")]
    Overflow(String),
    
    #[error("Duration underflow: {0}")]
    Underflow(String),
    
    #[error("Cannot parse duration: {0}")]
    CannotParseDuration(String),
    
    #[error("Invalid time component: {0}")]
    InvalidTimeComponent(String),
    
    #[error("Arithmetic error: {0}")]
    ArithmeticError(String),
}

impl DurationError {
    pub fn invalid_duration<S: Into<String>>(msg: S) -> Self {
        Self::InvalidDuration(msg.into())
    }
    
    pub fn overflow<S: Into<String>>(msg: S) -> Self {
        Self::Overflow(msg.into())
    }
    
    pub fn underflow<S: Into<String>>(msg: S) -> Self {
        Self::Underflow(msg.into())
    }
    
    pub fn cannot_parse_duration<S: Into<String>>(msg: S) -> Self {
        Self::CannotParseDuration(msg.into())
    }
    
    pub fn invalid_time_component<S: Into<String>>(msg: S) -> Self {
        Self::InvalidTimeComponent(msg.into())
    }
    
    pub fn arithmetic_error<S: Into<String>>(msg: S) -> Self {
        Self::ArithmeticError(msg.into())
    }
}