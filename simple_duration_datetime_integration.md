# Simple Duration and DateTime Integration

Based on your feedback, here are two focused, practical integration approaches:

## 1. **Duration Extraction from DateTime** - Simple & Useful

### Enhanced DateTime Methods for Duration Extraction

```rust
impl DateTime {
    // === Time-of-day extraction (already implemented) ===
    
    /// Get time elapsed since midnight as Duration
    pub fn time_since_midnight(&self) -> Duration {
        Duration::from_components(
            self.hour as u64,
            self.minute as u64,
            self.second as u64,
            0,
            self.nanosecond as u64
        )
    }
    
    /// Get time remaining until midnight as Duration
    pub fn time_until_midnight(&self) -> Duration {
        let midnight = Duration::from_hours(24);
        let current = self.time_since_midnight();
        midnight.subtract(&current).unwrap_or(Duration::zero())
    }
    
    // === New: More extraction methods ===
    
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
            .ok_or_else(|| /* error */)
    }
    
    /// Get Duration since start of month
    pub fn time_since_month_start(&self) -> Result<Duration> {
        let month_start = DateTime::from_date_start_of_day(
            self.year, 
            self.month, 
            Day::from_number(1)?
        )?;
        
        self.duration_since(&month_start)
            .ok_or_else(|| /* error */)
    }
    
    /// Get Duration since start of week (Monday)
    pub fn time_since_week_start(&self) -> Result<Duration> {
        let chrono_dt = self.to_chrono_naive()?;
        let weekday = chrono_dt.weekday();
        let days_since_monday = weekday.num_days_from_monday();
        
        let week_start = self.subtract_days(days_since_monday as u64)?;
        let week_start_dt = DateTime::from_date_start_of_day(week_start.0, week_start.1, week_start.2)?;
        
        self.duration_since(&week_start_dt)
            .ok_or_else(|| /* error */)
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
            .ok_or_else(|| /* error */)
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
            .ok_or_else(|| /* error */)
    }
}
```

### Usage Examples

```rust
let dt = DateTime::from("2024-03-15T14:30:45Z")?;

// Extract time components
let time_of_day = dt.extract_time();
println!("Time: {}", time_of_day.to_hms()); // "14:30:45"

// Time since various periods
let since_midnight = dt.time_since_midnight();
let since_month_start = dt.time_since_month_start()?;
let since_year_start = dt.time_since_year_start()?;

println!("Since midnight: {}", since_midnight.to_readable()); // "14h 30m 45s"
println!("Since month start: {}", since_month_start.to_readable()); // "14d 14h 30m 45s"

// Time until various periods
let until_midnight = dt.time_until_midnight();
let until_month_end = dt.time_until_month_end()?;

println!("Until midnight: {}", until_midnight.to_readable()); // "9h 29m 15s"
println!("Until month end: {}", until_month_end.to_readable());
```

## 2. **Simple Builder Pattern** - Ergonomic Construction

### DateTime Builder

```rust
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
        let year = self.year.ok_or_else(|| /* missing year error */)?;
        let month = self.month.ok_or_else(|| /* missing month error */)?;
        let day = self.day.ok_or_else(|| /* missing day error */)?;
        
        DateTime::new(year, month, day, self.hour, self.minute, self.second, self.nanosecond)
    }
}

// === Convenience constructor on DateTime ===

impl DateTime {
    pub fn builder() -> DateTimeBuilder {
        DateTimeBuilder::new()
    }
}
```

### Usage Examples

```rust
// Simple date with time
let dt1 = DateTime::builder()
    .date(2024, 3, 15)?
    .time(14, 30, 45)
    .build()?;

// Using Duration for time
let morning_time = Duration::from_components(9, 30, 0, 0, 0);
let dt2 = DateTime::builder()
    .date(2024, 3, 15)?
    .at_time(morning_time)
    .build()?;

// Convenience methods
let meeting_today = DateTime::builder()
    .today()?
    .hour(14)
    .minute(30)
    .build()?;

let lunch_tomorrow = DateTime::builder()
    .tomorrow()?
    .at_noon()
    .build()?;

// Using your existing types
let year = Year::from_number(2024)?;
let month = Month::from_number(3)?;
let day = Day::from_number(15)?;

let dt3 = DateTime::builder()
    .year(year)
    .month(month)
    .day(day)
    .time(16, 45, 0)
    .build()?;
```

## 3. **Combined Usage Patterns**

### Extract and Rebuild Pattern

```rust
// Extract time from one datetime and apply to another date
let source_dt = DateTime::from("2024-03-15T14:30:45Z")?;
let target_date = DateTime::from("2024-03-20")?;

let extracted_time = source_dt.extract_time();
let new_dt = DateTime::builder()
    .year(target_date.year)
    .month(target_date.month)
    .day(target_date.day)
    .at_time(extracted_time)
    .build()?;

println!("New datetime: {}", new_dt.to_iso8601()); // "2024-03-20T14:30:45.000000000Z"
```

### Time Calculations

```rust
let dt = DateTime::from("2024-03-15T14:30:45Z")?;

// How much time left in the day?
let remaining_today = dt.time_until_end_of_day();
println!("Time left today: {}", remaining_today.to_readable());

// How much time since start of week?
let since_week_start = dt.time_since_week_start()?;
println!("Since Monday: {}", since_week_start.to_readable());

// Create a datetime for same time tomorrow
let same_time_tomorrow = DateTime::builder()
    .tomorrow()?
    .at_time(dt.extract_time())
    .build()?;
```

## Benefits of This Simple Approach

1. **Easy Duration Extraction**: Get time spans from any DateTime easily
2. **Flexible Building**: Construct DateTimes in a readable, step-by-step way
3. **Practical Utilities**: Common operations like "same time tomorrow" or "time left today"
4. **Maintains Your API**: Still uses your existing Day/Month/Year types
5. **Simple to Understand**: No complex patterns, just straightforward methods

This approach gives you the most commonly needed functionality without overwhelming complexity. You can extract durations for calculations and build DateTimes in an intuitive way.