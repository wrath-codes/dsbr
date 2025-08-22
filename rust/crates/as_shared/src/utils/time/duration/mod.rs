use serde::{Serialize, Deserialize};
use crate::core::Result;
use crate::utils::{UtilsError};

pub mod error;
pub mod traits;
#[cfg(test)]
mod tests;

pub use error::DurationError;
pub use traits::{DurationValidatable, DurationFromInput};

/// Duration represents a time span with nanosecond precision
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct Duration {
    total_nanos: u64,
}

impl Duration {
    // === Constants ===
    
    const NANOS_PER_MICRO: u64 = 1_000;
    const NANOS_PER_MILLI: u64 = 1_000_000;
    const NANOS_PER_SECOND: u64 = 1_000_000_000;
    const NANOS_PER_MINUTE: u64 = 60 * Self::NANOS_PER_SECOND;
    const NANOS_PER_HOUR: u64 = 60 * Self::NANOS_PER_MINUTE;
    const NANOS_PER_DAY: u64 = 24 * Self::NANOS_PER_HOUR;
    
    // === Constructors ===
    
    /// Create a Duration from nanoseconds
    pub fn from_nanos(nanos: u64) -> Self {
        Self { total_nanos: nanos }
    }
    
    /// Create a Duration from microseconds
    pub fn from_micros(micros: u64) -> Self {
        Self { total_nanos: micros * Self::NANOS_PER_MICRO }
    }
    
    /// Create a Duration from milliseconds
    pub fn from_millis(millis: u64) -> Self {
        Self { total_nanos: millis * Self::NANOS_PER_MILLI }
    }
    
    /// Create a Duration from seconds
    pub fn from_seconds(seconds: u64) -> Self {
        Self { total_nanos: seconds * Self::NANOS_PER_SECOND }
    }
    
    /// Create a Duration from minutes
    pub fn from_minutes(minutes: u64) -> Self {
        Self { total_nanos: minutes * Self::NANOS_PER_MINUTE }
    }
    
    /// Create a Duration from hours
    pub fn from_hours(hours: u64) -> Self {
        Self { total_nanos: hours * Self::NANOS_PER_HOUR }
    }
    
    /// Create a Duration from days
    pub fn from_days(days: u64) -> Self {
        Self { total_nanos: days * Self::NANOS_PER_DAY }
    }
    
    /// Create a Duration from individual components
    pub fn from_components(hours: u64, minutes: u64, seconds: u64, millis: u64, nanos: u64) -> Self {
        let components = [
            (hours, Self::NANOS_PER_HOUR),
            (minutes, Self::NANOS_PER_MINUTE),
            (seconds, Self::NANOS_PER_SECOND),
            (millis, Self::NANOS_PER_MILLI),
            (nanos, 1),
        ];
        
        let total = components
            .iter()
            .map(|(value, multiplier)| value * multiplier)
            .sum();
            
        Self { total_nanos: total }
    }
    
    /// Create a zero duration
    pub fn zero() -> Self {
        Self { total_nanos: 0 }
    }
    
    /// Parse duration from any valid representation
    pub fn from<T>(input: T) -> Result<Duration>
    where
        T: DurationFromInput,
    {
        input.parse_duration()
    }
    
    // === Component extraction ===
    
    /// Get the hours component (0-23 for time-of-day, or total hours)
    pub fn hours(&self) -> u64 {
        (self.total_nanos / Self::NANOS_PER_HOUR) % 24
    }
    
    /// Get the minutes component (0-59)
    pub fn minutes(&self) -> u64 {
        (self.total_nanos % Self::NANOS_PER_HOUR) / Self::NANOS_PER_MINUTE
    }
    
    /// Get the seconds component (0-59)
    pub fn seconds(&self) -> u64 {
        (self.total_nanos % Self::NANOS_PER_MINUTE) / Self::NANOS_PER_SECOND
    }
    
    /// Get the milliseconds component (0-999)
    pub fn millis(&self) -> u64 {
        (self.total_nanos % Self::NANOS_PER_SECOND) / Self::NANOS_PER_MILLI
    }
    
    /// Get the microseconds component (0-999)
    pub fn micros(&self) -> u64 {
        (self.total_nanos % Self::NANOS_PER_MILLI) / Self::NANOS_PER_MICRO
    }
    
    /// Get the nanoseconds component (0-999)
    pub fn nanos(&self) -> u64 {
        self.total_nanos % Self::NANOS_PER_MICRO
    }
    
    // === Total conversions ===
    
    /// Get total duration as nanoseconds
    pub fn total_nanos(&self) -> u64 {
        self.total_nanos
    }
    
    /// Get total duration as microseconds
    pub fn total_micros(&self) -> u64 {
        self.total_nanos / Self::NANOS_PER_MICRO
    }
    
    /// Get total duration as milliseconds
    pub fn total_millis(&self) -> u64 {
        self.total_nanos / Self::NANOS_PER_MILLI
    }
    
    /// Get total duration as seconds
    pub fn total_seconds(&self) -> u64 {
        self.total_nanos / Self::NANOS_PER_SECOND
    }
    
    /// Get total duration as minutes
    pub fn total_minutes(&self) -> u64 {
        self.total_nanos / Self::NANOS_PER_MINUTE
    }
    
    /// Get total duration as hours
    pub fn total_hours(&self) -> u64 {
        self.total_nanos / Self::NANOS_PER_HOUR
    }
    
    /// Get total duration as days
    pub fn total_days(&self) -> u64 {
        self.total_nanos / Self::NANOS_PER_DAY
    }
    
    // === Arithmetic operations ===
    
    /// Add two durations
    pub fn add(&self, other: &Duration) -> Result<Duration> {
        self.total_nanos.checked_add(other.total_nanos)
            .map(|total| Duration { total_nanos: total })
            .ok_or_else(|| UtilsError::Duration(
                DurationError::overflow("Duration addition would overflow")
            ).into())
    }
    
    /// Subtract a duration from this one
    pub fn subtract(&self, other: &Duration) -> Result<Duration> {
        self.total_nanos.checked_sub(other.total_nanos)
            .map(|total| Duration { total_nanos: total })
            .ok_or_else(|| UtilsError::Duration(
                DurationError::underflow("Duration subtraction would underflow")
            ).into())
    }
    
    /// Multiply duration by a factor
    pub fn multiply(&self, factor: u64) -> Result<Duration> {
        self.total_nanos.checked_mul(factor)
            .map(|total| Duration { total_nanos: total })
            .ok_or_else(|| UtilsError::Duration(
                DurationError::overflow("Duration multiplication would overflow")
            ).into())
    }
    
    /// Divide duration by a divisor
    pub fn divide(&self, divisor: u64) -> Result<Duration> {
        match divisor {
            0 => Err(UtilsError::Duration(
                DurationError::arithmetic_error("Cannot divide duration by zero")
            ).into()),
            d => Ok(Duration { total_nanos: self.total_nanos / d }),
        }
    }
    
    // === Comparison methods ===
    
    /// Check if this duration is zero
    pub fn is_zero(&self) -> bool {
        self.total_nanos == 0
    }
    
    /// Check if this duration is longer than another
    pub fn is_longer_than(&self, other: &Duration) -> bool {
        self.total_nanos > other.total_nanos
    }
    
    /// Check if this duration is shorter than another
    pub fn is_shorter_than(&self, other: &Duration) -> bool {
        self.total_nanos < other.total_nanos
    }
    
    // === Formatting methods ===
    
    /// Format duration in a human-readable way
    pub fn to_readable(&self) -> String {
        let total_hours = self.total_hours();
        let m = self.minutes();
        let s = self.seconds();
        let ms = self.millis();
        
        match (total_hours, m, s, ms) {
            (h, _, _, _) if h >= 24 => {
                let days = self.total_days();
                let remaining_hours = h % 24;
                match (remaining_hours, m, s) {
                    (0, 0, 0) => format!("{}d", days),
                    _ => format!("{}d {}h {}m {}s", days, remaining_hours, m, s),
                }
            }
            (h, _, _, _) if h > 0 => format!("{}h {}m {}s", h, m, s),
            (0, m, _, _) if m > 0 => match (s, ms) {
                (0, 0) => format!("{}m", m),
                _ => format!("{}m {}s", m, s),
            },
            (0, 0, s, ms) if s > 0 => match ms {
                0 => format!("{}s", s),
                _ => format!("{}.{:03}s", s, ms),
            },
            (0, 0, 0, ms) if ms > 0 => format!("{}ms", ms),
            _ => {
                let total_micros = self.total_micros();
                match total_micros {
                    0 => format!("{}ns", self.total_nanos),
                    _ => format!("{}μs", total_micros),
                }
            }
        }
    }
    
    /// Format duration as HH:MM:SS
    pub fn to_hms(&self) -> String {
        let total_hours = self.total_hours();
        let m = self.minutes();
        let s = self.seconds();
        format!("{:02}:{:02}:{:02}", total_hours, m, s)
    }
    
    /// Format duration with full precision
    pub fn to_precise(&self) -> String {
        let total_hours = self.total_hours();
        let m = self.minutes();
        let s = self.seconds();
        let remaining_nanos = self.total_nanos % Self::NANOS_PER_SECOND;
        format!("{:02}:{:02}:{:02}.{:09}", total_hours, m, s, remaining_nanos)
    }
    
    /// Format duration as ISO 8601 duration string (P[n]Y[n]M[n]DT[n]H[n]M[n]S)
    pub fn to_iso8601(&self) -> String {
        let days = self.total_days();
        let hours = (self.total_nanos % Self::NANOS_PER_DAY) / Self::NANOS_PER_HOUR;
        let minutes = (self.total_nanos % Self::NANOS_PER_HOUR) / Self::NANOS_PER_MINUTE;
        let seconds = (self.total_nanos % Self::NANOS_PER_MINUTE) / Self::NANOS_PER_SECOND;
        let subsec_nanos = self.total_nanos % Self::NANOS_PER_SECOND;
        
        let mut components = Vec::new();
        
        // Add days component if present
        if days > 0 {
            components.push(format!("P{}DT", days));
        } else {
            components.push("PT".to_string());
        }
        
        // Add time components using functional approach
        [
            (hours, "H"),
            (minutes, "M"),
        ]
        .iter()
        .filter(|(value, _)| *value > 0)
        .for_each(|(value, unit)| components.push(format!("{}{}", value, unit)));
        
        // Handle seconds with potential fractional part
        match (seconds, subsec_nanos) {
            (0, 0) => {
                // Only add seconds if no other time components were added
                if components.len() == 1 {
                    components.push("0S".to_string());
                }
            }
            (s, 0) => components.push(format!("{}S", s)),
            (s, ns) => {
                let fractional = ns as f64 / Self::NANOS_PER_SECOND as f64;
                components.push(format!("{:.9}S", s as f64 + fractional));
            }
        }
        
        components.join("")
    }
    
    // === Parsing methods ===
    
    /// Parse duration string like "1h30m45s" or "2:30:15"
    pub fn parse_duration_string(input: &str) -> Result<Duration> {
        let input = input.trim();
        
        // Try HH:MM:SS format first
        if let Ok(duration) = Self::parse_hms_format(input) {
            return Ok(duration);
        }
        
        // Try component format like "1h30m45s"
        Self::parse_component_format(input)
    }
    
    /// Parse HH:MM:SS format
    fn parse_hms_format(input: &str) -> Result<Duration> {
        let mut parts = input.split(':');
        
        let (hours, minutes, seconds) = match (parts.next(), parts.next(), parts.next(), parts.next()) {
            (Some(h), Some(m), Some(s), None) => {
                let hours = h.parse::<u64>()
                    .map_err(|_| UtilsError::Duration(
                        DurationError::cannot_parse_duration("Invalid hours in HH:MM:SS format")
                    ))?;
                
                let minutes = m.parse::<u64>()
                    .map_err(|_| UtilsError::Duration(
                        DurationError::cannot_parse_duration("Invalid minutes in HH:MM:SS format")
                    ))?;
                
                let seconds = s.parse::<u64>()
                    .map_err(|_| UtilsError::Duration(
                        DurationError::cannot_parse_duration("Invalid seconds in HH:MM:SS format")
                    ))?;
                
                (hours, minutes, seconds)
            }
            _ => return Err(UtilsError::Duration(
                DurationError::cannot_parse_duration("Invalid HH:MM:SS format")
            ).into()),
        };
        
        match (minutes, seconds) {
            (m, s) if m >= 60 || s >= 60 => Err(UtilsError::Duration(
                DurationError::invalid_time_component("Minutes and seconds must be less than 60")
            ).into()),
            _ => Ok(Duration::from_components(hours, minutes, seconds, 0, 0)),
        }
    }
    
    /// Parse component format like "1h30m45s"
    fn parse_component_format(input: &str) -> Result<Duration> {
        use std::str::Chars;
        
        fn parse_number_and_unit(chars: &mut std::iter::Peekable<Chars>) -> Option<(u64, char)> {
            // Skip whitespace
            while chars.peek() == Some(&' ') {
                chars.next();
            }
            
            // Collect digits
            let mut number_str = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() {
                    number_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            
            if number_str.is_empty() {
                return None;
            }
            
            let value = number_str.parse::<u64>().ok()?;
            let unit = chars.next()?;
            
            Some((value, unit))
        }
        
        let mut chars = input.chars().peekable();
        let mut total_nanos = 0u64;
        
        while chars.peek().is_some() {
            let (value, unit) = parse_number_and_unit(&mut chars)
                .ok_or_else(|| UtilsError::Duration(
                    DurationError::cannot_parse_duration("Invalid format: expected number followed by unit")
                ))?;
            
            let multiplier = match unit.to_ascii_lowercase() {
                'd' => Self::NANOS_PER_DAY,
                'h' => Self::NANOS_PER_HOUR,
                'm' => Self::NANOS_PER_MINUTE,
                's' => Self::NANOS_PER_SECOND,
                _ => return Err(UtilsError::Duration(
                    DurationError::cannot_parse_duration(format!("Unknown time unit: {}", unit))
                ).into()),
            };
            
            let component_nanos = value.checked_mul(multiplier)
                .ok_or_else(|| UtilsError::Duration(
                    DurationError::overflow("Duration component would overflow")
                ))?;
            
            total_nanos = total_nanos.checked_add(component_nanos)
                .ok_or_else(|| UtilsError::Duration(
                    DurationError::overflow("Total duration would overflow")
                ))?;
        }
        
        Ok(Duration { total_nanos })
    }
    
    // === Conversion methods ===
    
    /// Convert to std::time::Duration
    pub fn to_std_duration(&self) -> std::time::Duration {
        std::time::Duration::new(
            self.total_seconds(),
            (self.total_nanos % Self::NANOS_PER_SECOND) as u32
        )
    }
    
    /// Convert to chrono::Duration
    pub fn to_chrono_duration(&self) -> chrono::Duration {
        chrono::Duration::nanoseconds(self.total_nanos as i64)
    }
    
    /// Validation methods
    pub fn is_valid<T: DurationValidatable>(input: T) -> bool {
        input.is_valid_duration()
    }
}

// === Default implementation ===
impl Default for Duration {
    fn default() -> Self {
        Self::zero()
    }
}

// === Display implementation ===
impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_readable())
    }
}