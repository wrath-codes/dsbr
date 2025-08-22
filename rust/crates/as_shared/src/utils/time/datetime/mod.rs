use chrono::{NaiveDateTime, DateTime as ChronoDateTime, Utc, Datelike, Timelike};
use crate::core::Result;
use crate::utils::{UtilsError};
use crate::utils::time::{Day, Month, Year, Duration};

pub mod error;
pub mod traits;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod integration_tests;

pub use error::DateTimeError;
pub use traits::{DateTimeValidatable, DateTimeFromInput};

/// Format enumeration for ergonomic API
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DateTimeFormat {
    ISO8601,           // 2024-03-15T14:30:45.123Z
    YYYYMMDD,          // 20240315
    YYYY_MM_DD,        // 2024-03-15
    DD_MM_YYYY,        // 15/03/2024
    MM_DD_YYYY,        // 03/15/2024
    DDMMYYYY,          // 15032024
    MMDDYYYY,          // 03152024
    YYMM,              // 2403
    Custom(String),    // Custom chrono pattern
}

/// DateTime represents a specific moment in time with nanosecond precision
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DateTime {
    pub year: Year,
    pub month: Month,
    pub day: Day,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nanosecond: u32,
}

impl DateTime {
    // === Core constructors ===
    
    /// Create a new DateTime with validation
    pub fn new(year: Year, month: Month, day: Day, hour: u8, minute: u8, second: u8, nanosecond: u32) -> Result<Self> {
        // Validate time components using match statements
        match hour {
            0..=23 => {},
            _ => return Err(UtilsError::DateTime(
                DateTimeError::invalid_time_component(format!("Hour must be 0-23, got {}", hour))
            ).into()),
        }
        
        match minute {
            0..=59 => {},
            _ => return Err(UtilsError::DateTime(
                DateTimeError::invalid_time_component(format!("Minute must be 0-59, got {}", minute))
            ).into()),
        }
        
        match second {
            0..=59 => {},
            _ => return Err(UtilsError::DateTime(
                DateTimeError::invalid_time_component(format!("Second must be 0-59, got {}", second))
            ).into()),
        }
        
        match nanosecond {
            0..=999_999_999 => {},
            _ => return Err(UtilsError::DateTime(
                DateTimeError::invalid_time_component(format!("Nanosecond must be 0-999999999, got {}", nanosecond))
            ).into()),
        }
        
        // Validate that the date is valid
        match day.is_valid_for_month(&month, &year) {
            true => Ok(Self { year, month, day, hour, minute, second, nanosecond }),
            false => Err(UtilsError::DateTime(
                DateTimeError::invalid_date_component(format!("Day {} is not valid for {} {}", day.day, month.to_en(), year.year))
            ).into()),
        }
    }
    
    /// Create from your existing types with time
    pub fn from_date_and_time(year: Year, month: Month, day: Day, hour: u8, minute: u8, second: u8) -> Result<Self> {
        Self::new(year, month, day, hour, minute, second, 0)
    }
    
    /// Create from your existing types at start of day
    pub fn from_date_start_of_day(year: Year, month: Month, day: Day) -> Result<Self> {
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse from any valid representation
    pub fn from<T>(input: T) -> Result<DateTime>
    where
        T: DateTimeFromInput,
    {
        input.parse_datetime()
    }
    
    // === UTC and Timestamp constructors ===
    
    /// Create DateTime from UTC timestamp (seconds since Unix epoch)
    pub fn from_timestamp(timestamp: i64) -> Result<Self> {
        let chrono_dt = ChronoDateTime::from_timestamp(timestamp, 0)
            .ok_or_else(|| UtilsError::DateTime(
                DateTimeError::timestamp_conversion(format!("Invalid timestamp: {}", timestamp))
            ))?;
        Self::from_chrono_utc(&chrono_dt)
    }
    
    /// Create DateTime from UTC timestamp with nanoseconds
    pub fn from_timestamp_nanos(timestamp: i64, nanos: u32) -> Result<Self> {
        let chrono_dt = ChronoDateTime::from_timestamp(timestamp, nanos)
            .ok_or_else(|| UtilsError::DateTime(
                DateTimeError::timestamp_conversion(format!("Invalid timestamp: {} with {} nanos", timestamp, nanos))
            ))?;
        Self::from_chrono_utc(&chrono_dt)
    }
    
    /// Create DateTime from current UTC time
    pub fn now_utc() -> Result<Self> {
        let now = Utc::now();
        Self::from_chrono_utc(&now)
    }
    
    /// Create DateTime from chrono UTC DateTime
    pub fn from_chrono_utc(dt: &ChronoDateTime<Utc>) -> Result<Self> {
        let naive = dt.naive_utc();
        Self::from_chrono_naive(&naive)
    }
    
    /// Create DateTime from chrono NaiveDateTime
    pub fn from_chrono_naive(dt: &NaiveDateTime) -> Result<Self> {
        let year = Year::from_naive_date(&dt.date())?;
        let month = Month::from_number(dt.month() as u8)?;
        let day = Day::from_naive_date(&dt.date())?;
        
        Self::new(
            year, month, day,
            dt.hour() as u8,
            dt.minute() as u8,
            dt.second() as u8,
            dt.nanosecond()
        )
    }
    
    // === Format-based parsing ===
    
    /// Parse from various date formats
    pub fn from_format(input: &str, format: DateTimeFormat) -> Result<Self> {
        match format {
            DateTimeFormat::ISO8601 => Self::from_iso8601(input),
            DateTimeFormat::YYYYMMDD => Self::from_yyyymmdd(input),
            DateTimeFormat::YYYY_MM_DD => Self::from_yyyy_mm_dd(input),
            DateTimeFormat::DD_MM_YYYY => Self::from_dd_mm_yyyy(input),
            DateTimeFormat::MM_DD_YYYY => Self::from_mm_dd_yyyy(input),
            DateTimeFormat::DDMMYYYY => Self::from_ddmmyyyy(input),
            DateTimeFormat::MMDDYYYY => Self::from_mmddyyyy(input),
            DateTimeFormat::YYMM => Self::from_yymm(input),
            DateTimeFormat::Custom(pattern) => Self::from_custom_format(input, &pattern),
        }
    }
    
    /// Parse ISO8601 format: "2024-03-15T14:30:45.123Z" or "2024-03-15T14:30:45"
    pub fn from_iso8601(input: &str) -> Result<Self> {
        let cleaned = input.trim_end_matches('Z');
        
        // Try parsing in order of specificity using functional approach
        let parse_attempts = [
            ("%Y-%m-%dT%H:%M:%S%.f", "with fractional seconds"),
            ("%Y-%m-%dT%H:%M:%S", "without fractional seconds"),
        ];
        
        // Try datetime formats first
        for (pattern, _description) in parse_attempts.iter() {
            if let Ok(naive) = NaiveDateTime::parse_from_str(cleaned, pattern) {
                return Self::from_chrono_naive(&naive);
            }
        }
        
        // Try date only format
        match chrono::NaiveDate::parse_from_str(cleaned, "%Y-%m-%d") {
            Ok(date) => {
                let naive = date.and_hms_opt(0, 0, 0)
                    .ok_or_else(|| UtilsError::DateTime(
                        DateTimeError::chrono_conversion("Failed to create time from date")
                    ))?;
                Self::from_chrono_naive(&naive)
            }
            Err(_) => Err(UtilsError::DateTime(
                DateTimeError::cannot_parse_datetime(format!("Invalid ISO8601 format: {}", input))
            ).into()),
        }
    }
    
    /// Parse YYYYMMDD format: "20240315" (assumes start of day)
    pub fn from_yyyymmdd(input: &str) -> Result<Self> {
        if input.len() != 8 {
            return Err(UtilsError::DateTime(
                DateTimeError::invalid_format("YYYYMMDD format must be exactly 8 digits")
            ).into());
        }
        
        let year_str = &input[0..4];
        let month_str = &input[4..6];
        let day_str = &input[6..8];
        
        let year = Year::from(year_str)?;
        let month = Month::from(month_str)?;
        let day = Day::from(day_str)?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse YYYY-MM-DD format: "2024-03-15" (assumes start of day)
    pub fn from_yyyy_mm_dd(input: &str) -> Result<Self> {
        let mut parts = input.split('-');
        
        let (year_str, month_str, day_str) = match (parts.next(), parts.next(), parts.next(), parts.next()) {
            (Some(y), Some(m), Some(d), None) => (y, m, d),
            _ => return Err(UtilsError::DateTime(
                DateTimeError::invalid_format("YYYY-MM-DD format must have exactly 3 parts separated by '-'")
            ).into()),
        };
        
        let year = Year::from(year_str)?;
        let month = Month::from(month_str)?;
        let day = Day::from(day_str)?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse DD/MM/YYYY format: "15/03/2024" (assumes start of day)
    pub fn from_dd_mm_yyyy(input: &str) -> Result<Self> {
        let mut parts = input.split('/');
        
        let (day_str, month_str, year_str) = match (parts.next(), parts.next(), parts.next(), parts.next()) {
            (Some(d), Some(m), Some(y), None) => (d, m, y),
            _ => return Err(UtilsError::DateTime(
                DateTimeError::invalid_format("DD/MM/YYYY format must have exactly 3 parts separated by '/'")
            ).into()),
        };
        
        let day = Day::from(day_str)?;
        let month = Month::from(month_str)?;
        let year = Year::from(year_str)?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse MM/DD/YYYY format: "03/15/2024" (assumes start of day)
    pub fn from_mm_dd_yyyy(input: &str) -> Result<Self> {
        let mut parts = input.split('/');
        
        let (month_str, day_str, year_str) = match (parts.next(), parts.next(), parts.next(), parts.next()) {
            (Some(m), Some(d), Some(y), None) => (m, d, y),
            _ => return Err(UtilsError::DateTime(
                DateTimeError::invalid_format("MM/DD/YYYY format must have exactly 3 parts separated by '/'")
            ).into()),
        };
        
        let month = Month::from(month_str)?;
        let day = Day::from(day_str)?;
        let year = Year::from(year_str)?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse DDMMYYYY format: "15032024" (assumes start of day)
    pub fn from_ddmmyyyy(input: &str) -> Result<Self> {
        if input.len() != 8 {
            return Err(UtilsError::DateTime(
                DateTimeError::invalid_format("DDMMYYYY format must be exactly 8 digits")
            ).into());
        }
        
        let day_str = &input[0..2];
        let month_str = &input[2..4];
        let year_str = &input[4..8];
        
        let day = Day::from(day_str)?;
        let month = Month::from(month_str)?;
        let year = Year::from(year_str)?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse MMDDYYYY format: "03152024" (assumes start of day)
    pub fn from_mmddyyyy(input: &str) -> Result<Self> {
        if input.len() != 8 {
            return Err(UtilsError::DateTime(
                DateTimeError::invalid_format("MMDDYYYY format must be exactly 8 digits")
            ).into());
        }
        
        let month_str = &input[0..2];
        let day_str = &input[2..4];
        let year_str = &input[4..8];
        
        let month = Month::from(month_str)?;
        let day = Day::from(day_str)?;
        let year = Year::from(year_str)?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse YYMM format: "2403" (assumes first day of month, start of day)
    pub fn from_yymm(input: &str) -> Result<Self> {
        if input.len() != 4 {
            return Err(UtilsError::DateTime(
                DateTimeError::invalid_format("YYMM format must be exactly 4 digits")
            ).into());
        }
        
        let year_str = &input[0..2];
        let month_str = &input[2..4];
        
        // Parse 2-digit year using existing pivot logic
        let year_2d = year_str.parse::<i32>()
            .map_err(|_| UtilsError::DateTime(
                DateTimeError::cannot_parse_datetime(format!("Invalid year in YYMM format: {}", year_str))
            ))?;
        let year = Year::from_2digit_number(year_2d)?;
        
        // Parse month
        let month = Month::from(month_str)?;
        
        // Default to first day of month, start of day
        let day = Day::from_number(1)?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse custom format using chrono patterns
    pub fn from_custom_format(input: &str, pattern: &str) -> Result<Self> {
        let naive = NaiveDateTime::parse_from_str(input, pattern)
            .map_err(|e| UtilsError::DateTime(
                DateTimeError::cannot_parse_datetime(format!("Failed to parse '{}' with pattern '{}': {}", input, pattern, e))
            ))?;
        Self::from_chrono_naive(&naive)
    }
    
    // === Accessors ===
    
    pub fn year(&self) -> &Year { &self.year }
    pub fn month(&self) -> &Month { &self.month }
    pub fn day(&self) -> &Day { &self.day }
    pub fn hour(&self) -> u8 { self.hour }
    pub fn minute(&self) -> u8 { self.minute }
    pub fn second(&self) -> u8 { self.second }
    pub fn nanosecond(&self) -> u32 { self.nanosecond }
    
    // === Duration arithmetic ===
    
    pub fn add_duration(&self, duration: &Duration) -> Result<Self> {
        // Convert current time to nanoseconds since start of day using functional approach
        let time_components = [
            (self.hour as u64, 3_600_000_000_000u64),
            (self.minute as u64, 60_000_000_000u64),
            (self.second as u64, 1_000_000_000u64),
            (self.nanosecond as u64, 1u64),
        ];
        
        let current_time_nanos = time_components
            .iter()
            .map(|(value, multiplier)| value * multiplier)
            .sum::<u64>();
        
        let total_nanos = current_time_nanos + duration.total_nanos();
        
        // Calculate days to add (if time overflows)
        const NANOS_PER_DAY: u64 = 24 * 3_600_000_000_000u64;
        let days_to_add = total_nanos / NANOS_PER_DAY;
        let remaining_nanos = total_nanos % NANOS_PER_DAY;
        
        // Extract new time components using functional approach
        let time_divisors = [3_600_000_000_000u64, 60_000_000_000u64, 1_000_000_000u64, 1u64];
        let mut remaining = remaining_nanos;
        let time_values: Vec<u64> = time_divisors
            .iter()
            .map(|&divisor| {
                let value = remaining / divisor;
                remaining %= divisor;
                value
            })
            .collect();
        
        let (new_hour, new_minute, new_second, new_nanosecond) =
            (time_values[0] as u8, time_values[1] as u8, time_values[2] as u8, time_values[3] as u32);
        
        // Handle date rollover
        let (new_year, new_month, new_day) = self.add_days(days_to_add)?;
        
        Self::new(new_year, new_month, new_day, new_hour, new_minute, new_second, new_nanosecond)
    }
    
    pub fn subtract_duration(&self, duration: &Duration) -> Result<Self> {
        let time_components = [
            (self.hour as u64, 3_600_000_000_000u64),
            (self.minute as u64, 60_000_000_000u64),
            (self.second as u64, 1_000_000_000u64),
            (self.nanosecond as u64, 1u64),
        ];
        
        let current_time_nanos = time_components
            .iter()
            .map(|(value, multiplier)| value * multiplier)
            .sum::<u64>();
        
        const NANOS_PER_DAY: u64 = 24 * 3_600_000_000_000u64;
        
        let extract_time_components = |nanos: u64| -> (u8, u8, u8, u32) {
            let time_divisors = [3_600_000_000_000u64, 60_000_000_000u64, 1_000_000_000u64, 1u64];
            let mut remaining = nanos;
            let time_values: Vec<u64> = time_divisors
                .iter()
                .map(|&divisor| {
                    let value = remaining / divisor;
                    remaining %= divisor;
                    value
                })
                .collect();
            (time_values[0] as u8, time_values[1] as u8, time_values[2] as u8, time_values[3] as u32)
        };
        
        match duration.total_nanos() <= current_time_nanos {
            true => {
                // Same day subtraction
                let remaining_nanos = current_time_nanos - duration.total_nanos();
                let (new_hour, new_minute, new_second, new_nanosecond) = extract_time_components(remaining_nanos);
                Self::new(self.year, self.month, self.day, new_hour, new_minute, new_second, new_nanosecond)
            }
            false => {
                // Need to go to previous day(s)
                let deficit = duration.total_nanos() - current_time_nanos;
                let days_to_subtract = (deficit / NANOS_PER_DAY) + 1;
                let remaining_nanos = NANOS_PER_DAY - (deficit % NANOS_PER_DAY);
                
                let (new_hour, new_minute, new_second, new_nanosecond) = extract_time_components(remaining_nanos);
                let (new_year, new_month, new_day) = self.subtract_days(days_to_subtract)?;
                
                Self::new(new_year, new_month, new_day, new_hour, new_minute, new_second, new_nanosecond)
            }
        }
    }
    
    // === Convenient arithmetic methods ===
    
    pub fn add_hours(&self, hours: u64) -> Result<Self> {
        self.add_duration(&Duration::from_hours(hours))
    }
    
    pub fn add_minutes(&self, minutes: u64) -> Result<Self> {
        self.add_duration(&Duration::from_minutes(minutes))
    }
    
    pub fn add_seconds(&self, seconds: u64) -> Result<Self> {
        self.add_duration(&Duration::from_seconds(seconds))
    }
    
    pub fn add_days(&self, days: u64) -> Result<(Year, Month, Day)> {
        let mut current_year = self.year;
        let mut current_month = self.month;
        let mut current_day = self.day;
        
        for _ in 0..days {
            if let Some(next_day) = current_day.next_in_month(&current_month, &current_year) {
                current_day = next_day;
            } else {
                // Move to next month
                current_month = current_month.next();
                current_day = Day::from_number(1)?;
                
                // Check if we need to move to next year
                if current_month.month == 1 {
                    current_year = current_year.next()?;
                }
            }
        }
        
        Ok((current_year, current_month, current_day))
    }
    
    pub fn subtract_days(&self, days: u64) -> Result<(Year, Month, Day)> {
        let mut current_year = self.year;
        let mut current_month = self.month;
        let mut current_day = self.day;
        
        for _ in 0..days {
            if let Some(prev_day) = current_day.previous_in_month(&current_month, &current_year) {
                current_day = prev_day;
            } else {
                // Move to previous month
                current_month = current_month.previous();
                
                // Check if we need to move to previous year
                if current_month.month == 12 {
                    current_year = current_year.previous()?;
                }
                
                // Get last day of the month
                let last_day_num = current_year.days_in_month(&current_month);
                current_day = Day::from_number(last_day_num)?;
            }
        }
        
        Ok((current_year, current_month, current_day))
    }
    
    // === Duration between DateTimes ===
    
    pub fn duration_since(&self, other: &DateTime) -> Option<Duration> {
        let self_total = self.total_nanos_since_epoch()?;
        let other_total = other.total_nanos_since_epoch()?;
        
        match self_total >= other_total {
            true => Some(Duration::from_nanos(self_total - other_total)),
            false => None,
        }
    }
    
    pub fn duration_until(&self, other: &DateTime) -> Option<Duration> {
        other.duration_since(self)
    }
    
    // === Time-of-day as Duration ===
    
    pub fn time_since_midnight(&self) -> Duration {
        Duration::from_components(
            self.hour as u64,
            self.minute as u64,
            self.second as u64,
            0, // millis
            self.nanosecond as u64
        )
    }
    
    pub fn time_until_midnight(&self) -> Duration {
        let midnight = Duration::from_hours(24);
        let current = self.time_since_midnight();
        midnight.subtract(&current).unwrap_or(Duration::zero())
    }
    
    // === Enhanced Duration extraction methods ===
    
    /// Extract just the time components as Duration (ignoring date)
    pub fn extract_time(&self) -> Duration {
        self.time_since_midnight()
    }
    
    /// Get Duration since start of year
    pub fn time_since_year_start(&self) -> Result<Duration> {
        let year_start = DateTime::from_date_start_of_day(
            self.year,
            Month::from_number(1)?,
            Day::from_number(1)?
        )?;
        
        self.duration_since(&year_start)
            .ok_or_else(|| UtilsError::DateTime(
                DateTimeError::arithmetic_underflow("DateTime is before year start")
            ).into())
    }
    
    /// Get Duration since start of month
    pub fn time_since_month_start(&self) -> Result<Duration> {
        let month_start = DateTime::from_date_start_of_day(
            self.year,
            self.month,
            Day::from_number(1)?
        )?;
        
        self.duration_since(&month_start)
            .ok_or_else(|| UtilsError::DateTime(
                DateTimeError::arithmetic_underflow("DateTime is before month start")
            ).into())
    }
    
    /// Get Duration since start of week (Monday)
    pub fn time_since_week_start(&self) -> Result<Duration> {
        let chrono_dt = self.to_chrono_naive()?;
        let weekday = chrono_dt.weekday();
        let days_since_monday = weekday.num_days_from_monday();
        
        let week_start = self.subtract_days(days_since_monday as u64)?;
        let week_start_dt = DateTime::from_date_start_of_day(week_start.0, week_start.1, week_start.2)?;
        
        self.duration_since(&week_start_dt)
            .ok_or_else(|| UtilsError::DateTime(
                DateTimeError::arithmetic_underflow("DateTime is before week start")
            ).into())
    }
    
    /// Get Duration until end of day
    pub fn time_until_end_of_day(&self) -> Duration {
        self.time_until_midnight()
    }
    
    /// Get Duration until end of month
    pub fn time_until_month_end(&self) -> Result<Duration> {
        let last_day_num = self.year.days_in_month(&self.month);
        let month_end = DateTime::new(
            self.year,
            self.month,
            Day::from_number(last_day_num)?,
            23, 59, 59, 999_999_999
        )?;
        
        self.duration_until(&month_end)
            .ok_or_else(|| UtilsError::DateTime(
                DateTimeError::arithmetic_overflow("DateTime is after month end")
            ).into())
    }
    
    /// Get Duration until end of year
    pub fn time_until_year_end(&self) -> Result<Duration> {
        let year_end = DateTime::new(
            self.year,
            Month::from_number(12)?,
            Day::from_number(31)?,
            23, 59, 59, 999_999_999
        )?;
        
        self.duration_until(&year_end)
            .ok_or_else(|| UtilsError::DateTime(
                DateTimeError::arithmetic_overflow("DateTime is after year end")
            ).into())
    }
    
    // === UTC and Timestamp conversion ===
    
    /// Convert to UTC timestamp (seconds since Unix epoch)
    pub fn to_timestamp(&self) -> Result<i64> {
        let chrono_dt = self.to_chrono_utc()?;
        Ok(chrono_dt.timestamp())
    }
    
    /// Convert to UTC timestamp with nanoseconds
    pub fn to_timestamp_nanos(&self) -> Result<(i64, u32)> {
        let chrono_dt = self.to_chrono_utc()?;
        Ok((chrono_dt.timestamp(), chrono_dt.timestamp_subsec_nanos()))
    }
    
    /// Convert to chrono UTC DateTime
    pub fn to_chrono_utc(&self) -> Result<ChronoDateTime<Utc>> {
        let naive = self.to_chrono_naive()?;
        Ok(naive.and_utc())
    }
    
    /// Convert to chrono NaiveDateTime
    pub fn to_chrono_naive(&self) -> Result<NaiveDateTime> {
        let naive_date = self.year.to_naive_date(&self.month, self.day.day as u32)?;
        let naive_time = chrono::NaiveTime::from_hms_nano_opt(
            self.hour as u32,
            self.minute as u32,
            self.second as u32,
            self.nanosecond
        ).ok_or_else(|| UtilsError::DateTime(
            DateTimeError::chrono_conversion("Failed to create NaiveTime from components")
        ))?;
        
        Ok(naive_date.and_time(naive_time))
    }
    
    // === Formatting ===
    
    /// Format to various standard formats
    pub fn to_format(&self, format: DateTimeFormat) -> Result<String> {
        match format {
            DateTimeFormat::ISO8601 => Ok(self.to_iso8601()),
            DateTimeFormat::YYYYMMDD => Ok(self.to_yyyymmdd()),
            DateTimeFormat::YYYY_MM_DD => Ok(self.to_yyyy_mm_dd()),
            DateTimeFormat::DD_MM_YYYY => Ok(self.to_dd_mm_yyyy()),
            DateTimeFormat::MM_DD_YYYY => Ok(self.to_mm_dd_yyyy()),
            DateTimeFormat::DDMMYYYY => Ok(self.to_ddmmyyyy()),
            DateTimeFormat::MMDDYYYY => Ok(self.to_mmddyyyy()),
            DateTimeFormat::YYMM => Ok(self.to_yymm()),
            DateTimeFormat::Custom(pattern) => self.to_custom_format(&pattern),
        }
    }
    
    pub fn to_iso8601(&self) -> String {
        format!("{}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}Z",
            self.year.year, self.month.month, self.day.day,
            self.hour, self.minute, self.second, self.nanosecond)
    }
    
    pub fn to_yyyymmdd(&self) -> String {
        format!("{}{:02}{:02}", self.year.year, self.month.month, self.day.day)
    }
    
    pub fn to_yyyy_mm_dd(&self) -> String {
        format!("{}-{:02}-{:02}", self.year.year, self.month.month, self.day.day)
    }
    
    pub fn to_dd_mm_yyyy(&self) -> String {
        format!("{:02}/{:02}/{}", self.day.day, self.month.month, self.year.year)
    }
    
    pub fn to_mm_dd_yyyy(&self) -> String {
        format!("{:02}/{:02}/{}", self.month.month, self.day.day, self.year.year)
    }
    
    pub fn to_ddmmyyyy(&self) -> String {
        format!("{:02}{:02}{}", self.day.day, self.month.month, self.year.year)
    }
    
    pub fn to_mmddyyyy(&self) -> String {
        format!("{:02}{:02}{}", self.month.month, self.day.day, self.year.year)
    }
    
    pub fn to_yymm(&self) -> String {
        format!("{}{:02}", self.year.to_2digit_text(), self.month.month)
    }
    
    pub fn to_custom_format(&self, pattern: &str) -> Result<String> {
        let naive = self.to_chrono_naive()?;
        Ok(naive.format(pattern).to_string())
    }
    
    // === Readable formatting ===
    
    pub fn to_readable_en(&self) -> String {
        format!("{} {}, {} at {:02}:{:02}:{:02}",
            self.month.to_en(), self.day.day, self.year.year,
            self.hour, self.minute, self.second)
    }
    
    pub fn to_readable_ptbr(&self) -> String {
        format!("{} de {} de {} às {:02}:{:02}:{:02}",
            self.day.day, self.month.to_ptbr(), self.year.year,
            self.hour, self.minute, self.second)
    }
    
    // === Helper methods ===
    
    fn total_nanos_since_epoch(&self) -> Option<u64> {
        // This is a simplified implementation
        // In a real implementation, you'd want to properly calculate from Unix epoch
        let chrono_dt = self.to_chrono_utc().ok()?;
        let timestamp = chrono_dt.timestamp();
        let nanos = chrono_dt.timestamp_subsec_nanos();
        
        match timestamp >= 0 {
            true => Some(timestamp as u64 * 1_000_000_000 + nanos as u64),
            false => None,
        }
    }
    
    /// Validation methods
    pub fn is_valid<T: DateTimeValidatable>(input: T) -> bool {
        input.is_valid_datetime()
    }
    
    /// Create a DateTime builder
    pub fn builder() -> DateTimeBuilder {
        DateTimeBuilder::new()
    }
}

// === Display implementation ===
impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_iso8601())
    }
}
/// DateTime Builder for ergonomic construction
pub struct DateTimeBuilder {
    year: Option<Year>,
    month: Option<Month>,
    day: Option<Day>,
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
}

impl DateTimeBuilder {
    pub fn new() -> Self {
        Self {
            year: None,
            month: None,
            day: None,
            hour: 0,
            minute: 0,
            second: 0,
            nanosecond: 0,
        }
    }
    
    // === Date setters ===
    
    pub fn year(mut self, year: Year) -> Self {
        self.year = Some(year);
        self
    }
    
    pub fn month(mut self, month: Month) -> Self {
        self.month = Some(month);
        self
    }
    
    pub fn day(mut self, day: Day) -> Self {
        self.day = Some(day);
        self
    }
    
    // === Time setters ===
    
    pub fn hour(mut self, hour: u8) -> Self {
        self.hour = hour;
        self
    }
    
    pub fn minute(mut self, minute: u8) -> Self {
        self.minute = minute;
        self
    }
    
    pub fn second(mut self, second: u8) -> Self {
        self.second = second;
        self
    }
    
    pub fn nanosecond(mut self, nanosecond: u32) -> Self {
        self.nanosecond = nanosecond;
        self
    }
    
    // === Convenience setters ===
    
    pub fn time(mut self, hour: u8, minute: u8, second: u8) -> Self {
        self.hour = hour;
        self.minute = minute;
        self.second = second;
        self
    }
    
    pub fn at_time(mut self, duration: Duration) -> Self {
        self.hour = duration.hours() as u8;
        self.minute = duration.minutes() as u8;
        self.second = duration.seconds() as u8;
        self.nanosecond = duration.nanos() as u32;
        self
    }
    
    pub fn at_noon(mut self) -> Self {
        self.hour = 12;
        self.minute = 0;
        self.second = 0;
        self.nanosecond = 0;
        self
    }
    
    pub fn at_midnight(mut self) -> Self {
        self.hour = 0;
        self.minute = 0;
        self.second = 0;
        self.nanosecond = 0;
        self
    }
    
    // === Date convenience methods ===
    
    pub fn today() -> Result<Self> {
        let now = DateTime::now_utc()?;
        Ok(Self::new()
            .year(now.year)
            .month(now.month)
            .day(now.day))
    }
    
    pub fn tomorrow() -> Result<Self> {
        let now = DateTime::now_utc()?;
        let tomorrow = now.add_days(1)?;
        Ok(Self::new()
            .year(tomorrow.0)
            .month(tomorrow.1)
            .day(tomorrow.2))
    }
    
    pub fn date(mut self, year: i32, month: u8, day: u8) -> Result<Self> {
        self.year = Some(Year::from_number(year)?);
        self.month = Some(Month::from_number(month)?);
        self.day = Some(Day::from_number(day)?);
        Ok(self)
    }
    
    // === Build method ===
    
    pub fn build(self) -> Result<DateTime> {
        let year = self.year.ok_or_else(|| UtilsError::DateTime(
            DateTimeError::invalid_date_component("Year is required")
        ))?;
        let month = self.month.ok_or_else(|| UtilsError::DateTime(
            DateTimeError::invalid_date_component("Month is required")
        ))?;
        let day = self.day.ok_or_else(|| UtilsError::DateTime(
            DateTimeError::invalid_date_component("Day is required")
        ))?;
        
        DateTime::new(year, month, day, self.hour, self.minute, self.second, self.nanosecond)
    }
}

impl Default for DateTimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}