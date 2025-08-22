use crate::core::Result;
use crate::utils::{UtilsError};
use crate::utils::time::datetime::{DateTime, DateTimeError};
use chrono::{NaiveDateTime, DateTime as ChronoDateTime, Utc};

/// Trait for types that can be validated as datetimes
pub trait DateTimeValidatable {
    fn is_valid_datetime(&self) -> bool;
}

impl DateTimeValidatable for str {
    fn is_valid_datetime(&self) -> bool {
        // Basic validation - check if it's not empty and contains some expected characters
        !self.is_empty() && (
            self.contains('-') || // ISO format
            self.contains('/') || // US/EU formats
            self.contains('T') || // ISO with time
            self.chars().all(|c| c.is_ascii_digit()) // Compact formats
        )
    }
}

impl DateTimeValidatable for &str {
    fn is_valid_datetime(&self) -> bool {
        (*self).is_valid_datetime()
    }
}

impl DateTimeValidatable for String {
    fn is_valid_datetime(&self) -> bool {
        self.as_str().is_valid_datetime()
    }
}

impl DateTimeValidatable for i64 {
    fn is_valid_datetime(&self) -> bool {
        // Basic timestamp validation - reasonable range
        *self >= 0 && *self <= 4_102_444_800 // Up to year 2100
    }
}

/// Trait for types that can be parsed into datetimes using the generic from() method
pub trait DateTimeFromInput {
    fn parse_datetime(self) -> Result<DateTime>;
}

impl DateTimeFromInput for &str {
    fn parse_datetime(self) -> Result<DateTime> {
        if !self.is_valid_datetime() {
            return Err(UtilsError::DateTime(
                DateTimeError::cannot_parse_datetime(format!("Unable to parse '{}' as a datetime", self))
            ).into());
        }
        
        // Try common formats in order of likelihood
        if let Ok(dt) = DateTime::from_iso8601(self) {
            return Ok(dt);
        }
        
        if let Ok(dt) = DateTime::from_yyyy_mm_dd(self) {
            return Ok(dt);
        }
        
        if let Ok(dt) = DateTime::from_dd_mm_yyyy(self) {
            return Ok(dt);
        }
        
        if let Ok(dt) = DateTime::from_mm_dd_yyyy(self) {
            return Ok(dt);
        }
        
        if let Ok(dt) = DateTime::from_yyyymmdd(self) {
            return Ok(dt);
        }
        
        if let Ok(dt) = DateTime::from_ddmmyyyy(self) {
            return Ok(dt);
        }
        
        if let Ok(dt) = DateTime::from_mmddyyyy(self) {
            return Ok(dt);
        }
        
        // Try YYMM format (4 digits, all numeric)
        if self.len() == 4 && self.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(dt) = DateTime::from_yymm(self) {
                return Ok(dt);
            }
        }
        
        Err(UtilsError::DateTime(
            DateTimeError::cannot_parse_datetime(format!("Unable to parse '{}' as a datetime", self))
        ).into())
    }
}

impl DateTimeFromInput for String {
    fn parse_datetime(self) -> Result<DateTime> {
        self.as_str().parse_datetime()
    }
}

impl DateTimeFromInput for &String {
    fn parse_datetime(self) -> Result<DateTime> {
        self.as_str().parse_datetime()
    }
}

impl DateTimeFromInput for i64 {
    fn parse_datetime(self) -> Result<DateTime> {
        if !self.is_valid_datetime() {
            return Err(UtilsError::DateTime(
                DateTimeError::timestamp_conversion(format!("Invalid timestamp: {}", self))
            ).into());
        }
        DateTime::from_timestamp(self)
    }
}

impl DateTimeFromInput for (i64, u32) {
    fn parse_datetime(self) -> Result<DateTime> {
        let (secs, nanos) = self;
        if !secs.is_valid_datetime() {
            return Err(UtilsError::DateTime(
                DateTimeError::timestamp_conversion(format!("Invalid timestamp: {}", secs))
            ).into());
        }
        DateTime::from_timestamp_nanos(secs, nanos)
    }
}

impl DateTimeFromInput for ChronoDateTime<Utc> {
    fn parse_datetime(self) -> Result<DateTime> {
        DateTime::from_chrono_utc(&self)
    }
}

impl DateTimeFromInput for &ChronoDateTime<Utc> {
    fn parse_datetime(self) -> Result<DateTime> {
        DateTime::from_chrono_utc(self)
    }
}

impl DateTimeFromInput for NaiveDateTime {
    fn parse_datetime(self) -> Result<DateTime> {
        DateTime::from_chrono_naive(&self)
    }
}

impl DateTimeFromInput for &NaiveDateTime {
    fn parse_datetime(self) -> Result<DateTime> {
        DateTime::from_chrono_naive(self)
    }
}

impl DateTimeFromInput for std::time::SystemTime {
    fn parse_datetime(self) -> Result<DateTime> {
        let duration_since_epoch = self.duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| UtilsError::DateTime(
                DateTimeError::timestamp_conversion(format!("SystemTime before Unix epoch: {}", e))
            ))?;
        
        let secs = duration_since_epoch.as_secs() as i64;
        let nanos = duration_since_epoch.subsec_nanos();
        
        DateTime::from_timestamp_nanos(secs, nanos)
    }
}