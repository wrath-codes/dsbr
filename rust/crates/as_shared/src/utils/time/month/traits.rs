use crate::core::Result;
use crate::utils::{UtilsError};
use crate::utils::time::month::{Month, MonthError};

/// Trait for types that can be validated as months
pub trait MonthValidatable {
    fn is_valid_month(&self) -> bool;
}

impl MonthValidatable for u8 {
    fn is_valid_month(&self) -> bool {
        *self >= 1 && *self <= 12
    }
}

impl MonthValidatable for str {
    fn is_valid_month(&self) -> bool {
        // Try all validation methods
        Month::is_valid_month_text(self) ||
        Month::is_valid_english_name(self) ||
        Month::is_valid_portuguese_name(self) ||
        Month::is_valid_abbreviation(self) ||
        Month::is_valid_month_number_string(self)
    }
}

impl MonthValidatable for &str {
    fn is_valid_month(&self) -> bool {
        // Try all validation methods
        Month::is_valid_month_text(self) ||
        Month::is_valid_english_name(self) ||
        Month::is_valid_portuguese_name(self) ||
        Month::is_valid_abbreviation(self) ||
        Month::is_valid_month_number_string(self)
    }
}

impl MonthValidatable for String {
    fn is_valid_month(&self) -> bool {
        self.as_str().is_valid_month()
    }
}

/// Trait for types that can be parsed into months using the generic from() method
pub trait MonthFromInput {
    fn parse_month(self) -> Result<Month>;
}

impl MonthFromInput for u8 {
    fn parse_month(self) -> Result<Month> {
        // Use the existing validation logic
        if !self.is_valid_month() {
            return Err(UtilsError::Month(
                MonthError::not_valid_month_number(format!("{}", self))
            ).into());
        }
        Month::from_number(self)
    }
}

impl MonthFromInput for &str {
    fn parse_month(self) -> Result<Month> {
        // Use the existing validation logic first
        if !self.is_valid_month() {
            return Err(UtilsError::Month(
                MonthError::cannot_parse_month(format!("Unable to parse '{}' as a month", self))
            ).into());
        }
        
        // Since validation passed, try parsing in order of specificity/performance:
        // 1. Zero-padded text (exact match, fastest)
        if let Ok(month) = Month::from_text(self) {
            return Ok(month);
        }
        
        // 2. Number string (simple parse)
        if let Ok(num) = self.parse::<u8>() {
            if let Ok(month) = Month::from_number(num) {
                return Ok(month);
            }
        }
        
        // 3. English name (common case)
        if let Ok(month) = Month::from_english_name(self) {
            return Ok(month);
        }
        
        // 4. Abbreviation (short strings)
        if let Ok(month) = Month::from_abbreviation(self) {
            return Ok(month);
        }
        
        // 5. Portuguese name (last resort)
        if let Ok(month) = Month::from_portuguese_name(self) {
            return Ok(month);
        }
        
        // This should never happen since validation passed, but just in case
        Err(UtilsError::Month(
            MonthError::cannot_parse_month(format!("Unable to parse '{}' as a month", self))
        ).into())
    }
}

impl MonthFromInput for String {
    fn parse_month(self) -> Result<Month> {
        self.as_str().parse_month()
    }
}

impl MonthFromInput for &String {
    fn parse_month(self) -> Result<Month> {
        self.as_str().parse_month()
    }
}
