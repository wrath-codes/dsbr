// Enhanced DateTime API with ergonomic parsing, formatting, and UTC/timestamp support

use crate::utils::time::{Day, Month, Year, Duration};
use chrono::{NaiveDateTime, DateTime as ChronoDateTime, TimeZone, Utc};
use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    
    pub fn new(year: Year, month: Month, day: Day, hour: u8, minute: u8, second: u8, nanosecond: u32) -> Result<Self> {
        // Validate time components
        if hour > 23 || minute > 59 || second > 59 || nanosecond >= 1_000_000_000 {
            return Err(/* time validation error */);
        }
        
        if !day.is_valid_for_month(&month, &year) {
            return Err(/* date validation error */);
        }
        
        Ok(Self { year, month, day, hour, minute, second, nanosecond })
    }
    
    // === UTC and Timestamp constructors (new ergonomic methods) ===
    
    /// Create DateTime from UTC timestamp (seconds since Unix epoch)
    pub fn from_timestamp(timestamp: i64) -> Result<Self> {
        let chrono_dt = ChronoDateTime::from_timestamp(timestamp, 0)
            .ok_or(/* invalid timestamp error */)?;
        Self::from_chrono_utc(&chrono_dt)
    }
    
    /// Create DateTime from UTC timestamp with nanoseconds
    pub fn from_timestamp_nanos(timestamp: i64, nanos: u32) -> Result<Self> {
        let chrono_dt = ChronoDateTime::from_timestamp(timestamp, nanos)
            .ok_or(/* invalid timestamp error */)?;
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
    
    // === Format-based parsing (new ergonomic methods) ===
    
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
            DateTimeFormat::Custom(pattern) => Self::from_custom_format(input, &pattern),
        }
    }
    
    /// Parse ISO8601 format: "2024-03-15T14:30:45.123Z" or "2024-03-15T14:30:45"
    pub fn from_iso8601(input: &str) -> Result<Self> {
        // Handle both with and without timezone/nanoseconds
        let cleaned = input.trim_end_matches('Z');
        
        if let Ok(naive) = NaiveDateTime::parse_from_str(cleaned, "%Y-%m-%dT%H:%M:%S%.f") {
            return Self::from_chrono_naive(&naive);
        }
        
        if let Ok(naive) = NaiveDateTime::parse_from_str(cleaned, "%Y-%m-%dT%H:%M:%S") {
            return Self::from_chrono_naive(&naive);
        }
        
        Err(/* parse error */)
    }
    
    /// Parse YYYYMMDD format: "20240315" (assumes start of day)
    pub fn from_yyyymmdd(input: &str) -> Result<Self> {
        if input.len() != 8 {
            return Err(/* invalid format error */);
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
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() != 3 {
            return Err(/* invalid format error */);
        }
        
        let year = Year::from(parts[0])?;
        let month = Month::from(parts[1])?;
        let day = Day::from(parts[2])?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse DD/MM/YYYY format: "15/03/2024" (assumes start of day)
    pub fn from_dd_mm_yyyy(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() != 3 {
            return Err(/* invalid format error */);
        }
        
        let day = Day::from(parts[0])?;
        let month = Month::from(parts[1])?;
        let year = Year::from(parts[2])?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse MM/DD/YYYY format: "03/15/2024" (assumes start of day)
    pub fn from_mm_dd_yyyy(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() != 3 {
            return Err(/* invalid format error */);
        }
        
        let month = Month::from(parts[0])?;
        let day = Day::from(parts[1])?;
        let year = Year::from(parts[2])?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse DDMMYYYY format: "15032024" (assumes start of day)
    pub fn from_ddmmyyyy(input: &str) -> Result<Self> {
        if input.len() != 8 {
            return Err(/* invalid format error */);
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
            return Err(/* invalid format error */);
        }
        
        let month_str = &input[0..2];
        let day_str = &input[2..4];
        let year_str = &input[4..8];
        
        let month = Month::from(month_str)?;
        let day = Day::from(day_str)?;
        let year = Year::from(year_str)?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
    
    /// Parse custom format using chrono patterns
    pub fn from_custom_format(input: &str, pattern: &str) -> Result<Self> {
        let naive = NaiveDateTime::parse_from_str(input, pattern)
            .map_err(|_| /* parse error */)?;
        Self::from_chrono_naive(&naive)
    }
    
    // === Enhanced arithmetic using Duration ===
    
    pub fn add_duration(&self, duration: &Duration) -> Result<Self> {
        // Implementation from previous example...
        todo!()
    }
    
    pub fn subtract_duration(&self, duration: &Duration) -> Result<Self> {
        // Implementation from previous example...
        todo!()
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
        ).ok_or(/* time error */)?;
        
        Ok(naive_date.and_time(naive_time))
    }
    
    // === Enhanced formatting ===
    
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
    
    pub fn to_custom_format(&self, pattern: &str) -> Result<String> {
        let naive = self.to_chrono_naive()?;
        Ok(naive.format(pattern).to_string())
    }
    
    // === Existing methods from previous examples ===
    // (accessors, readable formatting, etc.)
}

// === Format enumeration for ergonomic API ===

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimeFormat {
    ISO8601,           // 2024-03-15T14:30:45.123Z
    YYYYMMDD,          // 20240315
    YYYY_MM_DD,        // 2024-03-15
    DD_MM_YYYY,        // 15/03/2024
    MM_DD_YYYY,        // 03/15/2024
    DDMMYYYY,          // 15032024
    MMDDYYYY,          // 03152024
    Custom(String),    // Custom chrono pattern
}

// === Trait implementations for ergonomic parsing ===

pub trait DateTimeFromInput {
    fn parse_datetime(self) -> Result<DateTime>;
}

impl DateTimeFromInput for &str {
    fn parse_datetime(self) -> Result<DateTime> {
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
        
        Err(/* unable to parse error */)
    }
}

impl DateTimeFromInput for i64 {
    fn parse_datetime(self) -> Result<DateTime> {
        DateTime::from_timestamp(self)
    }
}

impl DateTimeFromInput for ChronoDateTime<Utc> {
    fn parse_datetime(self) -> Result<DateTime> {
        DateTime::from_chrono_utc(&self)
    }
}

impl DateTimeFromInput for NaiveDateTime {
    fn parse_datetime(self) -> Result<DateTime> {
        DateTime::from_chrono_naive(&self)
    }
}

// === Usage Examples ===

fn usage_examples() -> Result<()> {
    // === UTC and Timestamp operations ===
    
    // Current time
    let now = DateTime::now_utc()?;
    println!("Now: {}", now.to_iso8601());
    
    // From timestamp
    let from_ts = DateTime::from_timestamp(1710504645)?; // Unix timestamp
    let from_ts_nanos = DateTime::from_timestamp_nanos(1710504645, 123_456_789)?;
    
    // To timestamp
    let timestamp = now.to_timestamp()?;
    let (ts_secs, ts_nanos) = now.to_timestamp_nanos()?;
    
    // === Format-based parsing ===
    
    // Various date formats
    let dt1 = DateTime::from_format("2024-03-15T14:30:45Z", DateTimeFormat::ISO8601)?;
    let dt2 = DateTime::from_format("20240315", DateTimeFormat::YYYYMMDD)?;
    let dt3 = DateTime::from_format("2024-03-15", DateTimeFormat::YYYY_MM_DD)?;
    let dt4 = DateTime::from_format("15/03/2024", DateTimeFormat::DD_MM_YYYY)?;
    let dt5 = DateTime::from_format("03/15/2024", DateTimeFormat::MM_DD_YYYY)?;
    let dt6 = DateTime::from_format("15032024", DateTimeFormat::DDMMYYYY)?;
    let dt7 = DateTime::from_format("03152024", DateTimeFormat::MMDDYYYY)?;
    
    // Custom format
    let dt_custom = DateTime::from_format("15-Mar-2024 14:30", 
        DateTimeFormat::Custom("%d-%b-%Y %H:%M".to_string()))?;
    
    // === Ergonomic parsing (tries multiple formats) ===
    
    let dt_auto1 = DateTime::from("2024-03-15T14:30:45Z")?;
    let dt_auto2 = DateTime::from("2024-03-15")?;
    let dt_auto3 = DateTime::from("15/03/2024")?;
    let dt_auto4 = DateTime::from(1710504645i64)?; // timestamp
    
    // === Enhanced formatting ===
    
    println!("ISO8601: {}", dt1.to_format(DateTimeFormat::ISO8601)?);
    println!("YYYYMMDD: {}", dt1.to_format(DateTimeFormat::YYYYMMDD)?);
    println!("DD/MM/YYYY: {}", dt1.to_format(DateTimeFormat::DD_MM_YYYY)?);
    println!("Custom: {}", dt1.to_format(DateTimeFormat::Custom("%A, %B %d, %Y".to_string()))?);
    
    // === Duration arithmetic (from previous example) ===
    
    let duration = Duration::from_components(2, 30, 45, 0, 0); // 2h 30m 45s
    let later = dt1.add_duration(&duration)?;
    
    // === Integration with existing API ===
    
    // Your existing types still work
    println!("Year: {}", dt1.year().to_4digit_string());
    println!("Month: {}", dt1.month().to_en());
    println!("Day: {}", dt1.day().to_ordinal_en());
    
    Ok(())
}