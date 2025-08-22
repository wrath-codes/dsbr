# YYMM Format Enhancement for DateTime API

## Overview

Adding support for YYMM format (e.g., "2403" for March 2024) to the enhanced DateTime API. This format is commonly used in financial and business contexts where you need to represent a specific month/year combination.

## Design Changes

### 1. DateTimeFormat Enum Update

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimeFormat {
    ISO8601,           // 2024-03-15T14:30:45.123Z
    YYYYMMDD,          // 20240315
    YYYY_MM_DD,        // 2024-03-15
    DD_MM_YYYY,        // 15/03/2024
    MM_DD_YYYY,        // 03/15/2024
    DDMMYYYY,          // 15032024
    MMDDYYYY,          // 03152024
    YYMM,              // 2403 (NEW)
    Custom(String),    // Custom chrono pattern
}
```

### 2. YYMM Parsing Method

```rust
impl DateTime {
    /// Parse YYMM format: "2403" (assumes first day of month, start of day)
    /// Uses your existing Year pivot logic for 2-digit years
    pub fn from_yymm(input: &str) -> Result<Self> {
        if input.len() != 4 {
            return Err(/* invalid format error: must be exactly 4 digits */);
        }
        
        let year_str = &input[0..2];
        let month_str = &input[2..4];
        
        // Parse 2-digit year using existing pivot logic
        let year_2d = year_str.parse::<i32>()
            .map_err(|_| /* parse error */)?;
        let year = Year::from_2digit_number(year_2d)?;
        
        // Parse month
        let month = Month::from(month_str)?;
        
        // Default to first day of month, start of day
        let day = Day::from_number(1)?;
        
        Self::new(year, month, day, 0, 0, 0, 0)
    }
}
```

### 3. YYMM Formatting Method

```rust
impl DateTime {
    /// Format to YYMM: "2403" (2-digit year + 2-digit month)
    pub fn to_yymm(&self) -> String {
        format!("{}{:02}", self.year.to_2digit_text(), self.month.month)
    }
}
```

### 4. Updated Format Method

```rust
impl DateTime {
    pub fn to_format(&self, format: DateTimeFormat) -> Result<String> {
        match format {
            DateTimeFormat::ISO8601 => Ok(self.to_iso8601()),
            DateTimeFormat::YYYYMMDD => Ok(self.to_yyyymmdd()),
            DateTimeFormat::YYYY_MM_DD => Ok(self.to_yyyy_mm_dd()),
            DateTimeFormat::DD_MM_YYYY => Ok(self.to_dd_mm_yyyy()),
            DateTimeFormat::MM_DD_YYYY => Ok(self.to_mm_dd_yyyy()),
            DateTimeFormat::DDMMYYYY => Ok(self.to_ddmmyyyy()),
            DateTimeFormat::MMDDYYYY => Ok(self.to_mmddyyyy()),
            DateTimeFormat::YYMM => Ok(self.to_yymm()), // NEW
            DateTimeFormat::Custom(pattern) => self.to_custom_format(&pattern),
        }
    }
    
    pub fn from_format(input: &str, format: DateTimeFormat) -> Result<Self> {
        match format {
            DateTimeFormat::ISO8601 => Self::from_iso8601(input),
            DateTimeFormat::YYYYMMDD => Self::from_yyyymmdd(input),
            DateTimeFormat::YYYY_MM_DD => Self::from_yyyy_mm_dd(input),
            DateTimeFormat::DD_MM_YYYY => Self::from_dd_mm_yyyy(input),
            DateTimeFormat::MM_DD_YYYY => Self::from_mm_dd_yyyy(input),
            DateTimeFormat::DDMMYYYY => Self::from_ddmmyyyy(input),
            DateTimeFormat::MMDDYYYY => Self::from_mmddyyyy(input),
            DateTimeFormat::YYMM => Self::from_yymm(input), // NEW
            DateTimeFormat::Custom(pattern) => Self::from_custom_format(input, &pattern),
        }
    }
}
```

### 5. Updated Auto-Detection Logic

```rust
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
        
        // NEW: Try YYMM format (4 digits, all numeric)
        if self.len() == 4 && self.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(dt) = DateTime::from_yymm(self) {
                return Ok(dt);
            }
        }
        
        Err(/* unable to parse error */)
    }
}
```

## Usage Examples

```rust
fn yymm_usage_examples() -> Result<()> {
    // === YYMM Parsing ===
    
    // Explicit format parsing
    let dt1 = DateTime::from_format("2403", DateTimeFormat::YYMM)?;
    println!("Parsed: {}", dt1.to_iso8601()); // "2024-03-01T00:00:00.000000000Z"
    
    let dt2 = DateTime::from_format("9912", DateTimeFormat::YYMM)?; // December 1999 (pivot logic)
    let dt3 = DateTime::from_format("0501", DateTimeFormat::YYMM)?; // January 2005 (pivot logic)
    
    // Auto-detection (tries YYMM if 4 digits)
    let dt_auto = DateTime::from("2403")?;
    
    // === YYMM Formatting ===
    
    let year = Year::from_number(2024)?;
    let month = Month::from_number(3)?;
    let day = Day::from_number(15)?;
    let dt = DateTime::new(year, month, day, 14, 30, 0, 0)?;
    
    println!("YYMM: {}", dt.to_format(DateTimeFormat::YYMM)?); // "2403"
    println!("YYMM: {}", dt.to_yymm()); // "2403"
    
    // === Business Use Cases ===
    
    // Financial reporting periods
    let q1_2024 = DateTime::from_yymm("2401")?; // January 2024
    let q2_2024 = DateTime::from_yymm("2404")?; // April 2024
    
    // File naming with periods
    let report_period = DateTime::from_yymm("2403")?;
    let filename = format!("report_{}.csv", report_period.to_yymm()); // "report_2403.csv"
    
    // Batch processing by month
    let periods = vec!["2401", "2402", "2403", "2404"];
    for period in periods {
        let dt = DateTime::from_yymm(period)?;
        println!("Processing period: {} ({})", period, dt.month().to_en());
    }
    
    Ok(())
}
```

## Key Design Decisions

### 1. Default Behavior
- **Day**: Always defaults to 1st day of the month
- **Time**: Always defaults to start of day (00:00:00.000)
- **Rationale**: YYMM represents a month/year period, not a specific date/time

### 2. Year Pivot Logic
- Uses existing `Year::from_2digit_number()` method
- Leverages your established pivot logic for 2-digit years
- **Consistency**: Same behavior as other parts of your API

### 3. Auto-Detection Priority
- YYMM is tried last in auto-detection
- Only attempts YYMM if input is exactly 4 digits
- **Safety**: Prevents false positives with other 4-digit formats

### 4. Error Handling
- Follows your existing error patterns
- Validates year and month using existing validation logic
- **Consistency**: Same error types and messages as other parsing methods

## Integration Benefits

1. **Seamless Integration**: Uses your existing `Year`, `Month`, `Day` structs
2. **Consistent API**: Follows same patterns as other format methods
3. **Pivot Logic Reuse**: Leverages your established 2-digit year handling
4. **Business-Friendly**: Common format for financial/reporting systems
5. **Auto-Detection**: Works with generic `DateTime::from()` method

## Testing Considerations

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_yymm_parsing() {
        // Basic parsing
        let dt = DateTime::from_yymm("2403").unwrap();
        assert_eq!(dt.year().year, 2024);
        assert_eq!(dt.month().month, 3);
        assert_eq!(dt.day().day, 1);
        
        // Pivot logic
        let dt_99 = DateTime::from_yymm("9912").unwrap(); // 1999
        assert_eq!(dt_99.year().year, 1999);
        
        let dt_05 = DateTime::from_yymm("0501").unwrap(); // 2005
        assert_eq!(dt_05.year().year, 2005);
        
        // Error cases
        assert!(DateTime::from_yymm("240").is_err()); // Too short
        assert!(DateTime::from_yymm("24033").is_err()); // Too long
        assert!(DateTime::from_yymm("2413").is_err()); // Invalid month
        assert!(DateTime::from_yymm("ab03").is_err()); // Non-numeric
    }
    
    #[test]
    fn test_yymm_formatting() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(3).unwrap();
        let day = Day::from_number(15).unwrap();
        let dt = DateTime::new(year, month, day, 14, 30, 0, 0).unwrap();
        
        assert_eq!(dt.to_yymm(), "2403");
        assert_eq!(dt.to_format(DateTimeFormat::YYMM).unwrap(), "2403");
    }
    
    #[test]
    fn test_yymm_auto_detection() {
        let dt = DateTime::from("2403").unwrap();
        assert_eq!(dt.year().year, 2024);
        assert_eq!(dt.month().month, 3);
        assert_eq!(dt.day().day, 1);
    }
}
```

This enhancement maintains the ergonomic and consistent design of your DateTime API while adding the YYMM format support you requested.