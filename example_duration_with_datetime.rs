// Example showing how Duration works alongside DateTime for better time arithmetic

use crate::utils::time::{Day, Month, Year};
use serde::{Serialize, Deserialize};

// === Duration struct for time spans ===

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct Duration {
    total_nanos: u64,
}

impl Duration {
    // === Constructors ===
    
    pub fn from_hours(hours: u64) -> Self {
        Self { total_nanos: hours * 3_600_000_000_000 }
    }
    
    pub fn from_minutes(minutes: u64) -> Self {
        Self { total_nanos: minutes * 60_000_000_000 }
    }
    
    pub fn from_seconds(seconds: u64) -> Self {
        Self { total_nanos: seconds * 1_000_000_000 }
    }
    
    pub fn from_millis(millis: u64) -> Self {
        Self { total_nanos: millis * 1_000_000 }
    }
    
    pub fn from_nanos(nanos: u64) -> Self {
        Self { total_nanos: nanos }
    }
    
    pub fn from_components(hours: u64, minutes: u64, seconds: u64, millis: u64, nanos: u64) -> Self {
        let total = hours * 3_600_000_000_000 +
                   minutes * 60_000_000_000 +
                   seconds * 1_000_000_000 +
                   millis * 1_000_000 +
                   nanos;
        Self { total_nanos: total }
    }
    
    // === Component extraction ===
    
    pub fn hours(&self) -> u64 {
        self.total_nanos / 3_600_000_000_000
    }
    
    pub fn minutes(&self) -> u64 {
        (self.total_nanos % 3_600_000_000_000) / 60_000_000_000
    }
    
    pub fn seconds(&self) -> u64 {
        (self.total_nanos % 60_000_000_000) / 1_000_000_000
    }
    
    pub fn millis(&self) -> u64 {
        (self.total_nanos % 1_000_000_000) / 1_000_000
    }
    
    pub fn nanos(&self) -> u64 {
        self.total_nanos % 1_000_000
    }
    
    // === Total conversions ===
    
    pub fn total_hours(&self) -> u64 {
        self.total_nanos / 3_600_000_000_000
    }
    
    pub fn total_minutes(&self) -> u64 {
        self.total_nanos / 60_000_000_000
    }
    
    pub fn total_seconds(&self) -> u64 {
        self.total_nanos / 1_000_000_000
    }
    
    pub fn total_millis(&self) -> u64 {
        self.total_nanos / 1_000_000
    }
    
    pub fn total_nanos(&self) -> u64 {
        self.total_nanos
    }
    
    // === Arithmetic operations ===
    
    pub fn add(&self, other: &Duration) -> Duration {
        Duration { total_nanos: self.total_nanos + other.total_nanos }
    }
    
    pub fn subtract(&self, other: &Duration) -> Option<Duration> {
        if self.total_nanos >= other.total_nanos {
            Some(Duration { total_nanos: self.total_nanos - other.total_nanos })
        } else {
            None // Cannot have negative duration
        }
    }
    
    pub fn multiply(&self, factor: u64) -> Duration {
        Duration { total_nanos: self.total_nanos * factor }
    }
    
    pub fn divide(&self, divisor: u64) -> Duration {
        Duration { total_nanos: self.total_nanos / divisor }
    }
    
    // === Formatting ===
    
    pub fn to_readable(&self) -> String {
        let h = self.hours();
        let m = self.minutes();
        let s = self.seconds();
        let ms = self.millis();
        
        if h > 0 {
            format!("{}h {}m {}s", h, m, s)
        } else if m > 0 {
            format!("{}m {}s", m, s)
        } else if s > 0 {
            format!("{}.{:03}s", s, ms)
        } else {
            format!("{}ms", ms)
        }
    }
    
    pub fn to_hms(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hours(), self.minutes(), self.seconds())
    }
    
    pub fn to_precise(&self) -> String {
        format!("{:02}:{:02}:{:02}.{:09}", 
            self.hours(), self.minutes(), self.seconds(), self.nanos())
    }
}

// === Enhanced DateTime with Duration integration ===

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
    // ... (previous constructors remain the same)
    
    // === Enhanced time arithmetic using Duration ===
    
    pub fn add_duration(&self, duration: &Duration) -> Result<Self> {
        // Convert current time to nanoseconds since start of day
        let current_time_nanos = 
            self.hour as u64 * 3_600_000_000_000 +
            self.minute as u64 * 60_000_000_000 +
            self.second as u64 * 1_000_000_000 +
            self.nanosecond as u64;
        
        let total_nanos = current_time_nanos + duration.total_nanos();
        
        // Calculate days to add (if time overflows)
        let nanos_per_day = 24 * 3_600_000_000_000u64;
        let days_to_add = total_nanos / nanos_per_day;
        let remaining_nanos = total_nanos % nanos_per_day;
        
        // Extract new time components
        let new_hour = (remaining_nanos / 3_600_000_000_000) as u8;
        let new_minute = ((remaining_nanos % 3_600_000_000_000) / 60_000_000_000) as u8;
        let new_second = ((remaining_nanos % 60_000_000_000) / 1_000_000_000) as u8;
        let new_nanosecond = (remaining_nanos % 1_000_000_000) as u32;
        
        // Handle date rollover
        let (new_year, new_month, new_day) = self.add_days(days_to_add)?;
        
        Self::new(new_year, new_month, new_day, new_hour, new_minute, new_second, new_nanosecond)
    }
    
    pub fn subtract_duration(&self, duration: &Duration) -> Result<Self> {
        let current_time_nanos = 
            self.hour as u64 * 3_600_000_000_000 +
            self.minute as u64 * 60_000_000_000 +
            self.second as u64 * 1_000_000_000 +
            self.nanosecond as u64;
        
        if duration.total_nanos() <= current_time_nanos {
            // Same day subtraction
            let remaining_nanos = current_time_nanos - duration.total_nanos();
            let new_hour = (remaining_nanos / 3_600_000_000_000) as u8;
            let new_minute = ((remaining_nanos % 3_600_000_000_000) / 60_000_000_000) as u8;
            let new_second = ((remaining_nanos % 60_000_000_000) / 1_000_000_000) as u8;
            let new_nanosecond = (remaining_nanos % 1_000_000_000) as u32;
            
            Self::new(self.year, self.month, self.day, new_hour, new_minute, new_second, new_nanosecond)
        } else {
            // Need to go to previous day(s)
            let nanos_per_day = 24 * 3_600_000_000_000u64;
            let deficit = duration.total_nanos() - current_time_nanos;
            let days_to_subtract = (deficit / nanos_per_day) + 1;
            let remaining_nanos = nanos_per_day - (deficit % nanos_per_day);
            
            let new_hour = (remaining_nanos / 3_600_000_000_000) as u8;
            let new_minute = ((remaining_nanos % 3_600_000_000_000) / 60_000_000_000) as u8;
            let new_second = ((remaining_nanos % 60_000_000_000) / 1_000_000_000) as u8;
            let new_nanosecond = (remaining_nanos % 1_000_000_000) as u32;
            
            let (new_year, new_month, new_day) = self.subtract_days(days_to_subtract)?;
            
            Self::new(new_year, new_month, new_day, new_hour, new_minute, new_second, new_nanosecond)
        }
    }
    
    // === Duration between DateTimes ===
    
    pub fn duration_since(&self, other: &DateTime) -> Option<Duration> {
        let self_total = self.total_nanos_since_epoch()?;
        let other_total = other.total_nanos_since_epoch()?;
        
        if self_total >= other_total {
            Some(Duration::from_nanos(self_total - other_total))
        } else {
            None
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
        midnight.subtract(&current).unwrap_or(Duration::from_nanos(0))
    }
    
    // === Convenient arithmetic methods (using Duration internally) ===
    
    pub fn add_hours(&self, hours: u64) -> Result<Self> {
        self.add_duration(&Duration::from_hours(hours))
    }
    
    pub fn add_minutes(&self, minutes: u64) -> Result<Self> {
        self.add_duration(&Duration::from_minutes(minutes))
    }
    
    pub fn add_seconds(&self, seconds: u64) -> Result<Self> {
        self.add_duration(&Duration::from_seconds(seconds))
    }
    
    // === Helper methods ===
    
    fn add_days(&self, days: u64) -> Result<(Year, Month, Day)> {
        // Use your existing date navigation logic
        todo!("Implement using existing Day/Month/Year navigation")
    }
    
    fn subtract_days(&self, days: u64) -> Result<(Year, Month, Day)> {
        // Use your existing date navigation logic  
        todo!("Implement using existing Day/Month/Year navigation")
    }
    
    fn total_nanos_since_epoch(&self) -> Option<u64> {
        // Convert to some epoch (like Unix epoch) for comparison
        todo!("Implement epoch conversion")
    }
}

// === Usage Examples ===

fn usage_examples() -> Result<()> {
    let year = Year::from_number(2024)?;
    let month = Month::from_number(3)?;
    let day = Day::from_number(15)?;
    let dt = DateTime::new(year, month, day, 14, 30, 45, 0)?;
    
    // Create durations
    let two_hours = Duration::from_hours(2);
    let thirty_mins = Duration::from_minutes(30);
    let complex_duration = Duration::from_components(1, 45, 30, 500, 0); // 1h 45m 30.5s
    
    // Duration arithmetic
    let total_time = two_hours.add(&thirty_mins); // 2h 30m
    let half_time = total_time.divide(2); // 1h 15m
    
    println!("Duration: {}", total_time.to_readable()); // "2h 30m 0s"
    println!("Duration: {}", total_time.to_hms()); // "02:30:00"
    
    // DateTime arithmetic using Duration
    let later = dt.add_duration(&total_time)?; // Add 2h 30m
    let much_later = later.add_duration(&complex_duration)?; // Add 1h 45m 30.5s
    
    // Time spans between DateTimes
    if let Some(elapsed) = much_later.duration_since(&dt) {
        println!("Elapsed: {}", elapsed.to_readable()); // "4h 15m 30s"
    }
    
    // Time of day operations
    let time_since_midnight = dt.time_since_midnight();
    let time_until_midnight = dt.time_until_midnight();
    
    println!("Since midnight: {}", time_since_midnight.to_hms()); // "14:30:45"
    println!("Until midnight: {}", time_until_midnight.to_readable()); // "9h 29m 15s"
    
    // Convenient methods (using Duration internally)
    let tomorrow_same_time = dt.add_hours(24)?;
    let five_mins_later = dt.add_minutes(5)?;
    
    Ok(())
}