# Duration and DateTime Integration Suggestions

Based on the implemented API, here are several integration approaches for Duration and DateTime in your project:

## 1. **Operator Overloading Integration**

### Add/Subtract Operators
```rust
use std::ops::{Add, Sub};

impl Add<Duration> for DateTime {
    type Output = Result<DateTime>;
    
    fn add(self, duration: Duration) -> Self::Output {
        self.add_duration(&duration)
    }
}

impl Sub<Duration> for DateTime {
    type Output = Result<DateTime>;
    
    fn sub(self, duration: Duration) -> Self::Output {
        self.subtract_duration(&duration)
    }
}

impl Sub<DateTime> for DateTime {
    type Output = Option<Duration>;
    
    fn sub(self, other: DateTime) -> Self::Output {
        self.duration_since(&other)
    }
}

// Usage:
let dt = DateTime::now_utc()?;
let later = (dt + Duration::from_hours(2))?;
let elapsed = later - dt; // Returns Duration
```

## 2. **Builder Pattern Integration**

### DateTime Builder with Duration Components
```rust
pub struct DateTimeBuilder {
    year: Option<Year>,
    month: Option<Month>,
    day: Option<Day>,
    time_offset: Duration,
}

impl DateTimeBuilder {
    pub fn new() -> Self {
        Self {
            year: None,
            month: None,
            day: None,
            time_offset: Duration::zero(),
        }
    }
    
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
    
    pub fn at_time(mut self, duration: Duration) -> Self {
        self.time_offset = duration;
        self
    }
    
    pub fn at_hour(mut self, hour: u8) -> Self {
        self.time_offset = Duration::from_hours(hour as u64);
        self
    }
    
    pub fn plus_minutes(mut self, minutes: u64) -> Self {
        self.time_offset = self.time_offset.add(&Duration::from_minutes(minutes)).unwrap_or(self.time_offset);
        self
    }
    
    pub fn build(self) -> Result<DateTime> {
        let year = self.year.ok_or_else(|| /* error */)?;
        let month = self.month.ok_or_else(|| /* error */)?;
        let day = self.day.ok_or_else(|| /* error */)?;
        
        let base_dt = DateTime::from_date_start_of_day(year, month, day)?;
        base_dt.add_duration(&self.time_offset)
    }
}

// Usage:
let dt = DateTimeBuilder::new()
    .year(Year::from_number(2024)?)
    .month(Month::from_number(3)?)
    .day(Day::from_number(15)?)
    .at_hour(14)
    .plus_minutes(30)
    .build()?;
```

## 3. **Range and Interval Integration**

### DateTime Ranges with Duration
```rust
pub struct DateTimeRange {
    start: DateTime,
    end: DateTime,
}

impl DateTimeRange {
    pub fn new(start: DateTime, end: DateTime) -> Result<Self> {
        if start.duration_until(&end).is_some() {
            Ok(Self { start, end })
        } else {
            Err(/* invalid range error */)
        }
    }
    
    pub fn from_start_and_duration(start: DateTime, duration: Duration) -> Result<Self> {
        let end = start.add_duration(&duration)?;
        Ok(Self { start, end })
    }
    
    pub fn duration(&self) -> Duration {
        self.start.duration_until(&self.end).unwrap_or(Duration::zero())
    }
    
    pub fn contains(&self, dt: &DateTime) -> bool {
        self.start.duration_until(dt).is_some() && dt.duration_until(&self.end).is_some()
    }
    
    pub fn split_by_duration(&self, chunk_size: Duration) -> Vec<DateTimeRange> {
        let mut ranges = Vec::new();
        let mut current = self.start;
        
        while let Some(remaining) = current.duration_until(&self.end) {
            if remaining.is_longer_than(&chunk_size) {
                let chunk_end = current.add_duration(&chunk_size).unwrap();
                ranges.push(DateTimeRange::new(current, chunk_end).unwrap());
                current = chunk_end;
            } else {
                ranges.push(DateTimeRange::new(current, self.end).unwrap());
                break;
            }
        }
        
        ranges
    }
}

// Usage:
let start = DateTime::now_utc()?;
let range = DateTimeRange::from_start_and_duration(start, Duration::from_hours(8))?;
let hourly_chunks = range.split_by_duration(Duration::from_hours(1));
```

## 4. **Schedule and Recurring Events Integration**

### Recurring DateTime with Duration Intervals
```rust
pub struct RecurringDateTime {
    base: DateTime,
    interval: Duration,
    count: Option<usize>,
}

impl RecurringDateTime {
    pub fn every(interval: Duration) -> RecurringDateTimeBuilder {
        RecurringDateTimeBuilder::new(interval)
    }
    
    pub fn next_occurrence(&self, after: &DateTime) -> Option<DateTime> {
        if let Some(elapsed) = after.duration_since(&self.base) {
            let intervals_passed = elapsed.total_nanos() / self.interval.total_nanos();
            let next_interval = intervals_passed + 1;
            
            self.base.add_duration(&self.interval.multiply(next_interval)).ok()
        } else {
            Some(self.base)
        }
    }
    
    pub fn occurrences_in_range(&self, range: &DateTimeRange) -> Vec<DateTime> {
        let mut occurrences = Vec::new();
        let mut current = self.base;
        
        while range.contains(&current) {
            occurrences.push(current);
            if let Ok(next) = current.add_duration(&self.interval) {
                current = next;
            } else {
                break;
            }
            
            if let Some(max_count) = self.count {
                if occurrences.len() >= max_count {
                    break;
                }
            }
        }
        
        occurrences
    }
}

// Usage:
let daily_standup = RecurringDateTime::every(Duration::from_hours(24))
    .starting_at(DateTime::from("2024-03-15T09:00:00Z")?)
    .for_count(30)
    .build();

let this_week = DateTimeRange::from_start_and_duration(
    DateTime::now_utc()?,
    Duration::from_days(7)
)?;

let standups_this_week = daily_standup.occurrences_in_range(&this_week);
```

## 5. **Business Logic Integration**

### Working Hours and Business Days
```rust
pub struct BusinessHours {
    start_time: Duration, // Time since midnight
    end_time: Duration,   // Time since midnight
    working_days: Vec<chrono::Weekday>,
}

impl BusinessHours {
    pub fn standard() -> Self {
        Self {
            start_time: Duration::from_components(9, 0, 0, 0, 0), // 9:00 AM
            end_time: Duration::from_components(17, 0, 0, 0, 0),  // 5:00 PM
            working_days: vec![
                chrono::Weekday::Mon,
                chrono::Weekday::Tue,
                chrono::Weekday::Wed,
                chrono::Weekday::Thu,
                chrono::Weekday::Fri,
            ],
        }
    }
    
    pub fn is_business_time(&self, dt: &DateTime) -> Result<bool> {
        let weekday = dt.to_chrono_naive()?.weekday();
        if !self.working_days.contains(&weekday) {
            return Ok(false);
        }
        
        let time_of_day = dt.time_since_midnight();
        Ok(time_of_day.is_longer_than(&self.start_time) && 
           time_of_day.is_shorter_than(&self.end_time))
    }
    
    pub fn next_business_time(&self, from: &DateTime) -> Result<DateTime> {
        let mut candidate = *from;
        
        loop {
            if self.is_business_time(&candidate)? {
                return Ok(candidate);
            }
            
            // Move to next business day at start time
            candidate = candidate.add_days(1)?.0;
            candidate = DateTime::from_date_start_of_day(candidate.0, candidate.1, candidate.2)?
                .add_duration(&self.start_time)?;
        }
    }
    
    pub fn business_duration_between(&self, start: &DateTime, end: &DateTime) -> Result<Duration> {
        let mut total = Duration::zero();
        let mut current = *start;
        
        while let Some(remaining) = current.duration_until(end) {
            if remaining.is_zero() {
                break;
            }
            
            if self.is_business_time(&current)? {
                let day_end = current.add_duration(&self.end_time)?;
                let segment_end = if end.duration_since(&day_end).is_some() { day_end } else { *end };
                
                if let Some(segment_duration) = current.duration_until(&segment_end) {
                    total = total.add(&segment_duration)?;
                }
            }
            
            // Move to next day
            current = self.next_business_time(&current.add_days(1)?.0)?;
        }
        
        Ok(total)
    }
}

// Usage:
let business_hours = BusinessHours::standard();
let start = DateTime::from("2024-03-15T14:30:00Z")?;
let end = DateTime::from("2024-03-18T10:15:00Z")?;

let business_time_elapsed = business_hours.business_duration_between(&start, &end)?;
println!("Business hours: {}", business_time_elapsed.to_readable());
```

## 6. **Caching and Performance Integration**

### Duration-based Caching
```rust
pub struct TimestampedCache<T> {
    data: T,
    created_at: DateTime,
    ttl: Duration,
}

impl<T> TimestampedCache<T> {
    pub fn new(data: T, ttl: Duration) -> Result<Self> {
        Ok(Self {
            data,
            created_at: DateTime::now_utc()?,
            ttl,
        })
    }
    
    pub fn is_expired(&self) -> Result<bool> {
        let now = DateTime::now_utc()?;
        let age = self.created_at.duration_until(&now).unwrap_or(Duration::zero());
        Ok(age.is_longer_than(&self.ttl))
    }
    
    pub fn time_until_expiry(&self) -> Result<Duration> {
        let now = DateTime::now_utc()?;
        let age = self.created_at.duration_until(&now).unwrap_or(Duration::zero());
        
        if age.is_longer_than(&self.ttl) {
            Ok(Duration::zero())
        } else {
            self.ttl.subtract(&age).map_err(|e| e.into())
        }
    }
    
    pub fn get(&self) -> Result<Option<&T>> {
        if self.is_expired()? {
            Ok(None)
        } else {
            Ok(Some(&self.data))
        }
    }
}

// Usage:
let cached_data = TimestampedCache::new(
    "expensive_computation_result",
    Duration::from_minutes(30)
)?;

if let Some(data) = cached_data.get()? {
    println!("Using cached data: {}", data);
} else {
    println!("Cache expired, recomputing...");
}
```

## 7. **Logging and Metrics Integration**

### Performance Timing
```rust
pub struct Timer {
    start: DateTime,
    label: String,
}

impl Timer {
    pub fn start(label: impl Into<String>) -> Result<Self> {
        Ok(Self {
            start: DateTime::now_utc()?,
            label: label.into(),
        })
    }
    
    pub fn elapsed(&self) -> Result<Duration> {
        let now = DateTime::now_utc()?;
        self.start.duration_until(&now).ok_or_else(|| /* error */)
    }
    
    pub fn finish(self) -> Result<Duration> {
        let elapsed = self.elapsed()?;
        println!("[{}] completed in {}", self.label, elapsed.to_readable());
        Ok(elapsed)
    }
}

// Usage:
let timer = Timer::start("database_query")?;
// ... perform operation ...
let duration = timer.finish()?;

if duration.is_longer_than(&Duration::from_seconds(5)) {
    println!("Warning: Slow query detected!");
}
```

## Recommendation

I recommend starting with **Integration #1 (Operator Overloading)** and **Integration #3 (Range and Interval)** as they provide the most immediate ergonomic benefits and are commonly used patterns. Then gradually add the business logic integrations (#5) as needed for your specific use cases.

The operator overloading makes the API feel natural:
```rust
let meeting_start = DateTime::from("2024-03-15T14:00:00Z")?;
let meeting_end = meeting_start + Duration::from_hours(1)?;
let preparation_time = Duration::from_minutes(15);
let arrival_time = meeting_start - preparation_time?;
```

Which integration approach interests you most for your project's needs?