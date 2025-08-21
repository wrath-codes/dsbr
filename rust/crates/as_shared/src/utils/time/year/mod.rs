use dashmap::DashMap;
use std::sync::LazyLock;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, DateTime, TimeZone, Datelike};
use crate::core::Result;
use crate::utils::{UtilsError};
use crate::utils::time::Month;

pub mod error;
pub mod traits;

#[cfg(test)]
mod tests;

pub use error::{YearError, MIN_YEAR, MAX_YEAR, PIVOT_YEAR, CURRENT_CENTURY_START, PREVIOUS_CENTURY_START};
pub use traits::{YearValidatable, YearFromInput};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct Year {
    pub year: i32,
    pub text_2d: &'static str,
    pub text_4d: &'static str,
    pub is_leap: bool,
    pub century: u8,
    pub decade: u8,
}

/// Static storage for years (1900-2100)
pub static YEARS: LazyLock<DashMap<i32, Year>> = LazyLock::new(|| {
    let years = DashMap::with_capacity(201); // 1900-2100 = 201 years
    for year in MIN_YEAR..=MAX_YEAR {
        years.insert(year, Year::new_unchecked(year));
    }
    years
});

pub static YEARS_ORDERED: LazyLock<Vec<Year>> = LazyLock::new(|| {
    (MIN_YEAR..=MAX_YEAR).map(|y| Year::new_unchecked(y)).collect()
});

impl Year {
    /// Create a new Year without validation (internal use only)
    fn new_unchecked(year: i32) -> Self {
        let text_2d = match year % 100 {
            0..=9 => match year % 100 {
                0 => "00", 1 => "01", 2 => "02", 3 => "03", 4 => "04",
                5 => "05", 6 => "06", 7 => "07", 8 => "08", 9 => "09",
                _ => unreachable!(),
            },
            10..=99 => match year % 100 {
                10 => "10", 11 => "11", 12 => "12", 13 => "13", 14 => "14",
                15 => "15", 16 => "16", 17 => "17", 18 => "18", 19 => "19",
                20 => "20", 21 => "21", 22 => "22", 23 => "23", 24 => "24",
                25 => "25", 26 => "26", 27 => "27", 28 => "28", 29 => "29",
                30 => "30", 31 => "31", 32 => "32", 33 => "33", 34 => "34",
                35 => "35", 36 => "36", 37 => "37", 38 => "38", 39 => "39",
                40 => "40", 41 => "41", 42 => "42", 43 => "43", 44 => "44",
                45 => "45", 46 => "46", 47 => "47", 48 => "48", 49 => "49",
                50 => "50", 51 => "51", 52 => "52", 53 => "53", 54 => "54",
                55 => "55", 56 => "56", 57 => "57", 58 => "58", 59 => "59",
                60 => "60", 61 => "61", 62 => "62", 63 => "63", 64 => "64",
                65 => "65", 66 => "66", 67 => "67", 68 => "68", 69 => "69",
                70 => "70", 71 => "71", 72 => "72", 73 => "73", 74 => "74",
                75 => "75", 76 => "76", 77 => "77", 78 => "78", 79 => "79",
                80 => "80", 81 => "81", 82 => "82", 83 => "83", 84 => "84",
                85 => "85", 86 => "86", 87 => "87", 88 => "88", 89 => "89",
                90 => "90", 91 => "91", 92 => "92", 93 => "93", 94 => "94",
                95 => "95", 96 => "96", 97 => "97", 98 => "98", 99 => "99",
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        
        // For 4-digit text, we need to handle the full range
        let text_4d = Box::leak(year.to_string().into_boxed_str());
        
        let is_leap = Self::calculate_leap_year(year);
        let century = (year / 100) as u8;
        let decade = ((year % 100) / 10) as u8;
        
        Self {
            year,
            text_2d,
            text_4d,
            is_leap,
            century,
            decade,
        }
    }
    
    /// Calculate if a year is a leap year
    fn calculate_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
    
    /// Convert 2-digit year to 4-digit using pivot logic
    fn convert_2digit_to_4digit(year_2d: i32) -> i32 {
        if year_2d >= PIVOT_YEAR {
            PREVIOUS_CENTURY_START + year_2d
        } else {
            CURRENT_CENTURY_START + year_2d
        }
    }
    
    /// Returns all years in chronological order (1900 to 2100)
    pub fn all_years() -> &'static [Year] {
        &YEARS_ORDERED
    }
    
    /// Parse year from any valid representation
    ///
    /// This method attempts to parse the input using all available parsing methods:
    /// - Number parsing (for i32 values 1900-2100)
    /// - 4-digit string parsing (for strings like "2023", "1999")
    /// - 2-digit string parsing (for strings like "23", "99" with pivot logic)
    pub fn from<T>(input: T) -> Result<Year>
    where
        T: YearFromInput,
    {
        input.parse_year()
    }
    
    /// Find year by number (1900-2100)
    pub fn from_number(year: i32) -> Result<Year> {
        if !year.is_valid_year() {
            return Err(UtilsError::Year(
                YearError::invalid_year(year)
            ).into());
        }
        
        // Direct lookup from static collection - O(1)
        Ok(*YEARS.get(&year).unwrap())
    }
    
    /// Find year by 2-digit number with pivot logic
    pub fn from_2digit_number(year_2d: i32) -> Result<Year> {
        if !year_2d.is_valid_2digit_year() {
            return Err(UtilsError::Year(
                YearError::invalid_2digit_year(year_2d.to_string())
            ).into());
        }
        
        let full_year = Self::convert_2digit_to_4digit(year_2d);
        Self::from_number(full_year)
    }
    
    /// Get the next year
    pub fn next(&self) -> Result<Year> {
        let next_year = self.year + 1;
        if next_year > MAX_YEAR {
            return Err(UtilsError::Year(
                YearError::arithmetic_overflow(format!("Cannot get next year after {}", MAX_YEAR))
            ).into());
        }
        Self::from_number(next_year)
    }
    
    /// Get the previous year
    pub fn previous(&self) -> Result<Year> {
        let prev_year = self.year - 1;
        if prev_year < MIN_YEAR {
            return Err(UtilsError::Year(
                YearError::arithmetic_overflow(format!("Cannot get previous year before {}", MIN_YEAR))
            ).into());
        }
        Self::from_number(prev_year)
    }
    
    /// Add years to this year
    pub fn add_years(&self, years: i32) -> Result<Year> {
        let new_year = self.year + years;
        if new_year > MAX_YEAR {
            return Err(UtilsError::Year(
                YearError::arithmetic_overflow(format!("Adding {} years to {} would exceed maximum year {}", years, self.year, MAX_YEAR))
            ).into());
        }
        if new_year < MIN_YEAR {
            return Err(UtilsError::Year(
                YearError::arithmetic_overflow(format!("Adding {} years to {} would be below minimum year {}", years, self.year, MIN_YEAR))
            ).into());
        }
        Self::from_number(new_year)
    }
    
    /// Subtract years from this year
    pub fn subtract_years(&self, years: i32) -> Result<Year> {
        self.add_years(-years)
    }
    
    /// Check if this year comes before another chronologically
    pub fn is_before(&self, other: &Year) -> bool {
        self.year < other.year
    }
    
    /// Check if this year comes after another chronologically
    pub fn is_after(&self, other: &Year) -> bool {
        self.year > other.year
    }
    
    /// Get the number of years between this year and another
    /// (positive = forward, negative = backward)
    pub fn years_until(&self, other: &Year) -> i32 {
        other.year - self.year
    }
    
    /// Get the number of years since another year
    /// (positive = this year is later, negative = this year is earlier)
    pub fn years_since(&self, other: &Year) -> i32 {
        self.year - other.year
    }
    
    /// Convert to 2-digit string format ("00", "01", etc.)
    pub fn to_2digit_string(&self) -> String {
        self.text_2d.to_string()
    }
    
    /// Convert to 4-digit string format ("2023", "1999", etc.)
    pub fn to_4digit_string(&self) -> String {
        self.text_4d.to_string()
    }
    
    /// Convert to year number (1900-2100)
    pub fn to_number(&self) -> i32 {
        self.year
    }
    
    /// Get 2-digit text representation ("00", "01", etc.)
    pub fn to_2digit_text(&self) -> &'static str {
        self.text_2d
    }
    
    /// Get 4-digit text representation ("2023", "1999", etc.)
    pub fn to_4digit_text(&self) -> &'static str {
        self.text_4d
    }
    
    /// Check if this is a leap year
    pub fn is_leap_year(&self) -> bool {
        self.is_leap
    }
    
    /// Get the number of days in this year (365 or 366)
    pub fn days_in_year(&self) -> u16 {
        if self.is_leap {
            366
        } else {
            365
        }
    }
    
    /// Get the number of days in a specific month of this year
    pub fn days_in_month(&self, month: &Month) -> u8 {
        match month.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap { 29 } else { 28 },
            _ => unreachable!("Invalid month number"),
        }
    }
    
    /// Get the first day of the year (January 1st)
    pub fn year_start(&self) -> Result<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, 1, 1)
            .ok_or_else(|| UtilsError::Year(
                YearError::invalid_date(self.year, "Cannot create January 1st date")
            ).into())
    }
    
    /// Get the last day of the year (December 31st)
    pub fn year_end(&self) -> Result<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, 12, 31)
            .ok_or_else(|| UtilsError::Year(
                YearError::invalid_date(self.year, "Cannot create December 31st date")
            ).into())
    }
    
    /// Extract year from a NaiveDate
    pub fn from_naive_date(date: &NaiveDate) -> Result<Year> {
        let year_num = date.year();
        Self::from_number(year_num)
    }
    
    /// Extract year from a DateTime
    pub fn from_datetime<Tz: TimeZone>(datetime: &DateTime<Tz>) -> Result<Year> {
        let year_num = datetime.year();
        Self::from_number(year_num)
    }
    
    /// Create a NaiveDate from this year, month, and day
    pub fn to_naive_date(&self, month: &Month, day: u32) -> Result<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, month.month as u32, day)
            .ok_or_else(|| UtilsError::Year(
                YearError::invalid_date(self.year, format!("Cannot create date {}-{:02}-{:02}", self.year, month.month, day))
            ).into())
    }
    
    /// Create a DateTime<Utc> from this year, month, day, hour, minute, and second
    pub fn to_datetime_utc(&self, month: &Month, day: u32, hour: u32, min: u32, sec: u32) -> Result<DateTime<chrono::Utc>> {
        let naive_date = self.to_naive_date(month, day)?;
        let naive_time = chrono::NaiveTime::from_hms_opt(hour, min, sec)
            .ok_or_else(|| UtilsError::Year(
                YearError::invalid_date(self.year, format!("Cannot create time {:02}:{:02}:{:02}", hour, min, sec))
            ))?;
        
        Ok(naive_date.and_time(naive_time).and_utc())
    }
    
    /// Create a DateTime with a specific timezone
    pub fn to_datetime_with_timezone<Tz: TimeZone>(&self, month: &Month, day: u32, hour: u32, min: u32, sec: u32, tz: &Tz) -> Result<DateTime<Tz>> {
        let naive_date = self.to_naive_date(month, day)?;
        let naive_time = chrono::NaiveTime::from_hms_opt(hour, min, sec)
            .ok_or_else(|| UtilsError::Year(
                YearError::invalid_date(self.year, format!("Cannot create time {:02}:{:02}:{:02}", hour, min, sec))
            ))?;
        
        let naive_datetime = naive_date.and_time(naive_time);
        tz.from_local_datetime(&naive_datetime)
            .single()
            .ok_or_else(|| UtilsError::Year(
                YearError::chrono_conversion(format!("Cannot convert datetime to timezone: {}", naive_datetime))
            ).into())
    }
    
    /// Get the start of the year in a specific timezone
    pub fn year_start_in_timezone<Tz: TimeZone>(&self, tz: &Tz) -> Result<DateTime<Tz>> {
        let jan = Month::from_number(1).map_err(|e| UtilsError::Year(
            YearError::chrono_conversion(format!("Cannot get January: {}", e))
        ))?;
        self.to_datetime_with_timezone(&jan, 1, 0, 0, 0, tz)
    }
    
    /// Get the end of the year in a specific timezone
    pub fn year_end_in_timezone<Tz: TimeZone>(&self, tz: &Tz) -> Result<DateTime<Tz>> {
        let dec = Month::from_number(12).map_err(|e| UtilsError::Year(
            YearError::chrono_conversion(format!("Cannot get December: {}", e))
        ))?;
        self.to_datetime_with_timezone(&dec, 31, 23, 59, 59, tz)
    }
    
    /// Get the first day of a specific quarter
    pub fn quarter_start(&self, quarter: u8) -> Result<NaiveDate> {
        let month_num = match quarter {
            1 => 1,
            2 => 4,
            3 => 7,
            4 => 10,
            _ => return Err(UtilsError::Year(
                YearError::invalid_quarter(quarter)
            ).into()),
        };
        
        let month = Month::from_number(month_num).map_err(|e| UtilsError::Year(
            YearError::chrono_conversion(format!("Cannot get month {}: {}", month_num, e))
        ))?;
        
        self.to_naive_date(&month, 1)
    }
    
    /// Get the last day of a specific quarter
    pub fn quarter_end(&self, quarter: u8) -> Result<NaiveDate> {
        let (month_num, day) = match quarter {
            1 => (3, 31),
            2 => (6, 30),
            3 => (9, 30),
            4 => (12, 31),
            _ => return Err(UtilsError::Year(
                YearError::invalid_quarter(quarter)
            ).into()),
        };
        
        let month = Month::from_number(month_num).map_err(|e| UtilsError::Year(
            YearError::chrono_conversion(format!("Cannot get month {}: {}", month_num, e))
        ))?;
        
        self.to_naive_date(&month, day)
    }
    
    /// Get the first day of a specific month in this year
    pub fn month_start(&self, month: &Month) -> Result<NaiveDate> {
        self.to_naive_date(month, 1)
    }
    
    /// Get the last day of a specific month in this year
    pub fn month_end(&self, month: &Month) -> Result<NaiveDate> {
        let last_day = self.days_in_month(month);
        self.to_naive_date(month, last_day as u32)
    }
    
    /// Check if a specific date (month and day) is valid for this year
    pub fn is_valid_date(&self, month: &Month, day: u32) -> bool {
        if day == 0 {
            return false;
        }
        let max_days = self.days_in_month(month) as u32;
        day <= max_days
    }
    
    /// Get all months of this year with their start and end dates
    pub fn month_ranges(&self) -> Result<Vec<(Month, NaiveDate, NaiveDate)>> {
        let mut ranges = Vec::with_capacity(12);
        
        for month in Month::all_months() {
            let start = self.month_start(month)?;
            let end = self.month_end(month)?;
            ranges.push((*month, start, end));
        }
        
        Ok(ranges)
    }
    
    /// Get the quarter number (1-4) for a given month
    pub fn get_quarter(&self, month: &Month) -> u8 {
        match month.month {
            1..=3 => 1,
            4..=6 => 2,
            7..=9 => 3,
            10..=12 => 4,
            _ => unreachable!("Invalid month number"),
        }
    }
    
    /// Get all months in a specific quarter
    pub fn get_quarter_months(&self, quarter: u8) -> Result<Vec<Month>> {
        let month_range = match quarter {
            1 => 1..=3,
            2 => 4..=6,
            3 => 7..=9,
            4 => 10..=12,
            _ => return Err(UtilsError::Year(
                YearError::invalid_quarter(quarter)
            ).into()),
        };
        
        let mut months = Vec::with_capacity(3);
        for month_num in month_range {
            let month = Month::from_number(month_num).map_err(|e| UtilsError::Year(
                YearError::chrono_conversion(format!("Cannot get month {}: {}", month_num, e))
            ))?;
            months.push(month);
        }
        
        Ok(months)
    }
    
    /// Public generic validation method - accepts different year types
    pub fn is_valid<T: YearValidatable>(input: T) -> bool {
        input.is_valid_year()
    }
}