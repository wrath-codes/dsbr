use crate::core::Result;
use crate::utils::{UtilsError};
use crate::utils::time::duration::{Duration, DurationError};

/// Trait for types that can be validated as durations
pub trait DurationValidatable {
    fn is_valid_duration(&self) -> bool;
}

impl DurationValidatable for u64 {
    fn is_valid_duration(&self) -> bool {
        // All u64 values are valid for nanoseconds
        true
    }
}

impl DurationValidatable for str {
    fn is_valid_duration(&self) -> bool {
        // Basic validation - check if it's a valid duration string format
        // This is a simplified check, actual parsing will do more thorough validation
        !self.is_empty() && self.chars().any(|c| c.is_ascii_digit())
    }
}

impl DurationValidatable for &str {
    fn is_valid_duration(&self) -> bool {
        (*self).is_valid_duration()
    }
}

impl DurationValidatable for String {
    fn is_valid_duration(&self) -> bool {
        self.as_str().is_valid_duration()
    }
}

/// Trait for types that can be parsed into durations using the generic from() method
pub trait DurationFromInput {
    fn parse_duration(self) -> Result<Duration>;
}

impl DurationFromInput for u64 {
    fn parse_duration(self) -> Result<Duration> {
        Ok(Duration::from_nanos(self))
    }
}

impl DurationFromInput for &str {
    fn parse_duration(self) -> Result<Duration> {
        if !self.is_valid_duration() {
            return Err(UtilsError::Duration(
                DurationError::cannot_parse_duration(format!("Unable to parse '{}' as a duration", self))
            ).into());
        }
        
        // Try to parse as nanoseconds first
        if let Ok(nanos) = self.parse::<u64>() {
            return Ok(Duration::from_nanos(nanos));
        }
        
        // Try to parse duration string formats like "1h30m45s"
        Duration::parse_duration_string(self)
    }
}

impl DurationFromInput for String {
    fn parse_duration(self) -> Result<Duration> {
        self.as_str().parse_duration()
    }
}

impl DurationFromInput for &String {
    fn parse_duration(self) -> Result<Duration> {
        self.as_str().parse_duration()
    }
}

impl DurationFromInput for std::time::Duration {
    fn parse_duration(self) -> Result<Duration> {
        let total_nanos = self.as_secs() * 1_000_000_000 + self.subsec_nanos() as u64;
        Ok(Duration::from_nanos(total_nanos))
    }
}

impl DurationFromInput for chrono::Duration {
    fn parse_duration(self) -> Result<Duration> {
        let total_nanos = self.num_nanoseconds()
            .ok_or_else(|| UtilsError::Duration(
                DurationError::overflow("Chrono duration too large to convert to nanoseconds")
            ))?;
        
        if total_nanos < 0 {
            return Err(UtilsError::Duration(
                DurationError::underflow("Cannot create Duration from negative chrono::Duration")
            ).into());
        }
        
        Ok(Duration::from_nanos(total_nanos as u64))
    }
}