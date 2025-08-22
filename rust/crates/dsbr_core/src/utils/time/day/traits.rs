use crate::core::Result;
use crate::utils::{UtilsError};
use crate::utils::time::day::{Day, DayError};

/// Trait for types that can be validated as days
pub trait DayValidatable {
    fn is_valid_day(&self) -> bool;
}

impl DayValidatable for u8 {
    fn is_valid_day(&self) -> bool {
        *self >= 1 && *self <= 31
    }
}

impl DayValidatable for str {
    fn is_valid_day(&self) -> bool {
        Day::is_valid_day_string(self)
    }
}

impl DayValidatable for &str {
    fn is_valid_day(&self) -> bool {
        (*self).is_valid_day()
    }
}

impl DayValidatable for String {
    fn is_valid_day(&self) -> bool {
        self.as_str().is_valid_day()
    }
}

/// Trait for types that can be parsed into days using the generic from() method
pub trait DayFromInput {
    fn parse_day(self) -> Result<Day>;
}

impl DayFromInput for u8 {
    fn parse_day(self) -> Result<Day> {
        if !self.is_valid_day() {
            return Err(UtilsError::Day(
                DayError::invalid_day(self)
            ).into());
        }
        Day::from_number(self)
    }
}

impl DayFromInput for &str {
    fn parse_day(self) -> Result<Day> {
        if !self.is_valid_day() {
            return Err(UtilsError::Day(
                DayError::cannot_parse_day(format!("Unable to parse '{}' as a day", self))
            ).into());
        }
        
        if let Ok(num) = self.parse::<u8>() {
            Day::from_number(num)
        } else {
            Err(UtilsError::Day(
                DayError::cannot_parse_day(format!("Unable to parse '{}' as a day", self))
            ).into())
        }
    }
}

impl DayFromInput for String {
    fn parse_day(self) -> Result<Day> {
        self.as_str().parse_day()
    }
}

impl DayFromInput for &String {
    fn parse_day(self) -> Result<Day> {
        self.as_str().parse_day()
    }
}