# Month Operations

The [`Month`](https://docs.rs/arrow-sus-shared/latest/arrow_sus_shared/utils/time/month/struct.Month.html) utility provides comprehensive month manipulation capabilities with support for multiple languages and formats.

## Overview

The `Month` struct represents a calendar month with the following properties:

- **Numeric representation** (1-12)
- **Zero-padded text** ("01", "02", etc.)
- **English names** ("January", "February", etc.)
- **Portuguese names** ("Janeiro", "Fevereiro", etc.)
- **Abbreviations** ("Jan", "Feb", etc.)

## Creating Months

### From Numbers

```rust
use arrow_sus_shared::utils::time::month::Month;

// Create from month number (1-12)
let january = Month::from_number(1)?;
let december = Month::from_number(12)?;

// Invalid numbers return errors
assert!(Month::from_number(0).is_err());
assert!(Month::from_number(13).is_err());
```

### From Text Representations

```rust
// From zero-padded strings
let jan = Month::from_text("01")?;
let dec = Month::from_text("12")?;

// From English names (case-insensitive)
let february = Month::from_english_name("February")?;
let march = Month::from_english_name("march")?;

// From Portuguese names (case-insensitive)
let janeiro = Month::from_portuguese_name("Janeiro")?;
let fevereiro = Month::from_portuguese_name("fevereiro")?;

// From abbreviations (case-insensitive)
let apr = Month::from_abbreviation("Apr")?;
let may = Month::from_abbreviation("mai")?; // Portuguese abbreviation
```

## Navigation

### Next and Previous Months

```rust
let january = Month::from_number(1)?;
let february = january.next();
let december = january.previous(); // Wraps around

assert_eq!(february.month, 2);
assert_eq!(december.month, 12);
```

### Month Calculations

```rust
let jan = Month::from_number(1)?;
let mar = Month::from_number(3)?;
let nov = Month::from_number(11)?;

// Calculate months between
let forward = jan.months_until(&mar);   // 2
let backward = mar.months_until(&jan);  // -2

// Calculate months since
let since = mar.months_since(&jan);     // 2
let since_back = jan.months_since(&mar); // -2

// Comparison operations
assert!(jan.is_before(&mar));
assert!(nov.is_after(&jan));
```

## Validation

### Using the MonthValidatable Trait

```rust
use arrow_sus_shared::utils::time::month::{Month, MonthValidatable};

// Validate different types
assert!(1u8.is_valid_month());
assert!(!13u8.is_valid_month());

assert!("January".is_valid_month());
assert!("01".is_valid_month());
assert!("Jan".is_valid_month());
assert!("Janeiro".is_valid_month());
assert!(!"Invalid".is_valid_month());

// Generic validation
assert!(Month::is_valid(5u8));
assert!(Month::is_valid("February"));
assert!(!Month::is_valid("NotAMonth"));
```

## Output Formats

### String Representations

```rust
let march = Month::from_number(3)?;

// Different output formats
println!("{}", march.to_zero_padded_string()); // "03"
println!("{}", march.to_number_string());      // "3"
println!("{}", march.text);                    // "03"
println!("{}", march.name_en);                 // "March"
println!("{}", march.name_ptbr);               // "Março"
println!("{}", march.name_short);              // "Mar"
```

## Working with All Months

### Iterating Over Months

```rust
// Get all months in chronological order
let all_months = Month::all_months();

for month in all_months {
    println!("{}: {} ({})", 
        month.month, 
        month.name_en, 
        month.name_ptbr
    );
}
```

### Seasonal Operations

```rust
// Example: Get all spring months (Northern Hemisphere)
let spring_months: Vec<Month> = Month::all_months()
    .iter()
    .filter(|m| m.month >= 3 && m.month <= 5)
    .copied()
    .collect();

// Example: Get all months with 31 days
let long_months = [1, 3, 5, 7, 8, 10, 12];
let months_31_days: Vec<Month> = Month::all_months()
    .iter()
    .filter(|m| long_months.contains(&m.month))
    .copied()
    .collect();
```

## Error Handling

The Month utilities use comprehensive error types:

```rust
use arrow_sus_shared::utils::time::month::MonthError;

match Month::from_number(15) {
    Ok(month) => println!("Valid month: {}", month.name_en),
    Err(e) => match e.downcast_ref::<MonthError>() {
        Some(MonthError::NotValidMonthNumber(msg)) => {
            println!("Invalid month number: {}", msg);
        }
        _ => println!("Other error: {}", e),
    }
}
```

## Performance Notes

- **Month creation from numbers**: O(1) - Direct array indexing
- **Month creation from text/names**: O(1) - Small fixed-size lookups
- **Navigation operations**: O(1) - Simple arithmetic
- **Validation**: O(1) - Efficient checks

The `Month` utilities are designed for high-performance applications with minimal overhead.

## Integration Examples

### With Date Libraries

```rust
use chrono::{NaiveDate, Datelike};
use arrow_sus_shared::utils::time::month::Month;

// Convert from chrono
let chrono_date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
let month = Month::from_number(chrono_date.month() as u8)?;

println!("Month: {} ({})", month.name_en, month.name_ptbr);
```

### With Serialization

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DateRange {
    start_month: u8,
    end_month: u8,
}

impl DateRange {
    fn get_months(&self) -> Result<Vec<Month>, Box<dyn std::error::Error>> {
        let start = Month::from_number(self.start_month)?;
        let end = Month::from_number(self.end_month)?;
        
        let mut months = Vec::new();
        let mut current = start;
        
        loop {
            months.push(current);
            if current.month == end.month {
                break;
            }
            current = current.next();
        }
        
        Ok(months)
    }
}