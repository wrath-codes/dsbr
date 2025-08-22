use crate::core::Result;
use crate::utils::UtilsError;
use super::{Year, YearError};
use super::error::{MIN_YEAR, MAX_YEAR};

/// Trait for types that can be validated as years
pub trait YearValidatable {
    fn is_valid_year(&self) -> bool;
    fn is_valid_2digit_year(&self) -> bool;
    fn is_valid_4digit_year(&self) -> bool;
}

impl YearValidatable for i32 {
    fn is_valid_year(&self) -> bool {
        *self >= MIN_YEAR && *self <= MAX_YEAR
    }
    
    fn is_valid_2digit_year(&self) -> bool {
        *self >= 0 && *self <= 99
    }
    
    fn is_valid_4digit_year(&self) -> bool {
        self.is_valid_year()
    }
}

impl YearValidatable for str {
    fn is_valid_year(&self) -> bool {
        // Try all validation methods
        self.is_valid_4digit_year() || self.is_valid_2digit_year()
    }
    
    fn is_valid_2digit_year(&self) -> bool {
        if self.len() != 2 {
            return false;
        }
        if let Ok(year) = self.parse::<i32>() {
            year.is_valid_2digit_year()
        } else {
            false
        }
    }
    
    fn is_valid_4digit_year(&self) -> bool {
        if self.len() != 4 {
            return false;
        }
        if let Ok(year) = self.parse::<i32>() {
            year.is_valid_year()
        } else {
            false
        }
    }
}

impl YearValidatable for &str {
    fn is_valid_year(&self) -> bool {
        (*self).is_valid_year()
    }
    
    fn is_valid_2digit_year(&self) -> bool {
        (*self).is_valid_2digit_year()
    }
    
    fn is_valid_4digit_year(&self) -> bool {
        (*self).is_valid_4digit_year()
    }
}

impl YearValidatable for String {
    fn is_valid_year(&self) -> bool {
        self.as_str().is_valid_year()
    }
    
    fn is_valid_2digit_year(&self) -> bool {
        self.as_str().is_valid_2digit_year()
    }
    
    fn is_valid_4digit_year(&self) -> bool {
        self.as_str().is_valid_4digit_year()
    }
}

/// Trait for types that can be parsed into years using the generic from() method
pub trait YearFromInput {
    fn parse_year(self) -> Result<Year>;
}

impl YearFromInput for i32 {
    fn parse_year(self) -> Result<Year> {
        if !self.is_valid_year() {
            return Err(UtilsError::Year(
                YearError::invalid_year(self)
            ).into());
        }
        Year::from_number(self)
    }
}

impl YearFromInput for &str {
    fn parse_year(self) -> Result<Year> {
        if !self.is_valid_year() {
            return Err(UtilsError::Year(
                YearError::cannot_parse_year(format!("Unable to parse '{}' as a year", self))
            ).into());
        }
        
        // Try 4-digit first (most common case)
        if self.is_valid_4digit_year() {
            if let Ok(year) = self.parse::<i32>() {
                return Year::from_number(year);
            }
        }
        
        // Try 2-digit
        if self.is_valid_2digit_year() {
            if let Ok(year_2d) = self.parse::<i32>() {
                return Year::from_2digit_number(year_2d);
            }
        }
        
        // This should never happen since validation passed, but just in case
        Err(UtilsError::Year(
            YearError::cannot_parse_year(format!("Unable to parse '{}' as a year", self))
        ).into())
    }
}

impl YearFromInput for String {
    fn parse_year(self) -> Result<Year> {
        self.as_str().parse_year()
    }
}

impl YearFromInput for &String {
    fn parse_year(self) -> Result<Year> {
        self.as_str().parse_year()
    }
}