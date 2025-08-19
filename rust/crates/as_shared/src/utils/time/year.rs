use dashmap::DashMap;
use std::sync::LazyLock;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use chrono::{NaiveDate, DateTime, TimeZone, Datelike};
use crate::core::Result;
use crate::utils::UtilsError;
use crate::utils::time::Month;

/// Year validation constants
pub const MIN_YEAR: i32 = 1900;
pub const MAX_YEAR: i32 = 2100;
pub const PIVOT_YEAR: i32 = 50;  // 2-digit years 00-49 = 2000-2049, 50-99 = 1950-1999
pub const CURRENT_CENTURY_START: i32 = 2000;
pub const PREVIOUS_CENTURY_START: i32 = 1900;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct Year {
    pub year: i32,
    pub text_2d: &'static str,
    pub text_4d: &'static str,
    pub is_leap: bool,
    pub century: u8,
    pub decade: u8,
}

#[derive(Error, Debug)]
pub enum YearError {
    #[error("Invalid year: {0}. Must be between {1} and {2}")]
    InvalidYear(i32, i32, i32),
    
    #[error("Invalid 2-digit year: {0}. Must be between 00 and 99")]
    Invalid2DigitYear(String),
    
    #[error("Invalid 4-digit year: {0}. Must be between {1} and {2}")]
    Invalid4DigitYear(String, i32, i32),
    
    #[error("Cannot parse year: {0}")]
    CannotParseYear(String),
    
    #[error("Chrono conversion error: {0}")]
    ChronoConversion(String),
    
    #[error("Year arithmetic overflow: {0}")]
    ArithmeticOverflow(String),
    
    #[error("Invalid date for year {0}: {1}")]
    InvalidDate(i32, String),
    
    #[error("Invalid quarter: {0}. Must be between 1 and 4")]
    InvalidQuarter(u8),
    
    #[error("Not a valid year: {0}. This value cannot be converted to a year.")]
    NotValidYear(String),
}

impl YearError {
    pub fn invalid_year(year: i32) -> Self {
        Self::InvalidYear(year, MIN_YEAR, MAX_YEAR)
    }
    
    pub fn invalid_2digit_year<S: Into<String>>(year: S) -> Self {
        Self::Invalid2DigitYear(year.into())
    }
    
    pub fn invalid_4digit_year<S: Into<String>>(year: S) -> Self {
        Self::Invalid4DigitYear(year.into(), MIN_YEAR, MAX_YEAR)
    }
    
    pub fn cannot_parse_year<S: Into<String>>(msg: S) -> Self {
        Self::CannotParseYear(msg.into())
    }
    
    pub fn chrono_conversion<S: Into<String>>(msg: S) -> Self {
        Self::ChronoConversion(msg.into())
    }
    
    pub fn arithmetic_overflow<S: Into<String>>(msg: S) -> Self {
        Self::ArithmeticOverflow(msg.into())
    }
    
    pub fn invalid_date<S: Into<String>>(year: i32, msg: S) -> Self {
        Self::InvalidDate(year, msg.into())
    }
    
    pub fn invalid_quarter(quarter: u8) -> Self {
        Self::InvalidQuarter(quarter)
    }
    
    pub fn not_valid_year<S: Into<String>>(msg: S) -> Self {
        Self::NotValidYear(msg.into())
    }
}

/// Trait for types that can be validated as years
/// 
/// # Examples
/// 
/// ```
/// use arrow_sus_shared::utils::time::year::YearValidatable;
/// 
/// // i32 validation
/// assert!(2023i32.is_valid_year());
/// assert!(!1800i32.is_valid_year());
/// 
/// // str validation
/// assert!("2023".is_valid_year());
/// assert!("23".is_valid_2digit_year());
/// assert!(!"invalid".is_valid_year());
/// 
/// // String validation
/// assert!(String::from("2024").is_valid_year());
/// ```
pub trait YearValidatable {
    fn is_valid_year(&self) -> bool;
    fn is_valid_2digit_year(&self) -> bool;
    fn is_valid_4digit_year(&self) -> bool;
}

impl YearValidatable for i32 {
    fn is_valid_year(&self) -> bool {
        *self >= MIN_YEAR && *self <= MAX_YEAR
    }
    
    fn is_valid_2digit_year(&self) -> bool {
        *self >= 0 && *self <= 99
    }
    
    fn is_valid_4digit_year(&self) -> bool {
        self.is_valid_year()
    }
}

impl YearValidatable for str {
    fn is_valid_year(&self) -> bool {
        // Try all validation methods
        self.is_valid_4digit_year() || self.is_valid_2digit_year()
    }
    
    fn is_valid_2digit_year(&self) -> bool {
        if self.len() != 2 {
            return false;
        }
        if let Ok(year) = self.parse::<i32>() {
            year.is_valid_2digit_year()
        } else {
            false
        }
    }
    
    fn is_valid_4digit_year(&self) -> bool {
        if self.len() != 4 {
            return false;
        }
        if let Ok(year) = self.parse::<i32>() {
            year.is_valid_year()
        } else {
            false
        }
    }
}

impl YearValidatable for &str {
    fn is_valid_year(&self) -> bool {
        (*self).is_valid_year()
    }
    
    fn is_valid_2digit_year(&self) -> bool {
        (*self).is_valid_2digit_year()
    }
    
    fn is_valid_4digit_year(&self) -> bool {
        (*self).is_valid_4digit_year()
    }
}

impl YearValidatable for String {
    fn is_valid_year(&self) -> bool {
        self.as_str().is_valid_year()
    }
    
    fn is_valid_2digit_year(&self) -> bool {
        self.as_str().is_valid_2digit_year()
    }
    
    fn is_valid_4digit_year(&self) -> bool {
        self.as_str().is_valid_4digit_year()
    }
}

/// Trait for types that can be parsed into years using the generic from() method
///
/// # Examples
///
/// ```
/// use arrow_sus_shared::utils::time::Year;
/// use arrow_sus_shared::utils::time::year::YearFromInput;
///
/// // i32 parsing
/// assert!(2023i32.parse_year().is_ok());
/// assert!(1800i32.parse_year().is_err());
///
/// // str parsing
/// assert!("2023".parse_year().is_ok());
/// assert!("23".parse_year().is_ok()); // 2-digit year
/// assert!("invalid".parse_year().is_err());
///
/// // String parsing
/// assert!(String::from("2024").parse_year().is_ok());
/// ```
pub trait YearFromInput {
    fn parse_year(self) -> Result<Year>;
}

impl YearFromInput for i32 {
    fn parse_year(self) -> Result<Year> {
        if !self.is_valid_year() {
            return Err(UtilsError::Year(
                YearError::invalid_year(self)
            ).into());
        }
        Year::from_number(self)
    }
}

impl YearFromInput for &str {
    fn parse_year(self) -> Result<Year> {
        if !self.is_valid_year() {
            return Err(UtilsError::Year(
                YearError::cannot_parse_year(format!("Unable to parse '{}' as a year", self))
            ).into());
        }
        
        // Try 4-digit first (most common case)
        if self.is_valid_4digit_year() {
            if let Ok(year) = self.parse::<i32>() {
                return Year::from_number(year);
            }
        }
        
        // Try 2-digit
        if self.is_valid_2digit_year() {
            if let Ok(year_2d) = self.parse::<i32>() {
                return Year::from_2digit_number(year_2d);
            }
        }
        
        // This should never happen since validation passed, but just in case
        Err(UtilsError::Year(
            YearError::cannot_parse_year(format!("Unable to parse '{}' as a year", self))
        ).into())
    }
}

impl YearFromInput for String {
    fn parse_year(self) -> Result<Year> {
        self.as_str().parse_year()
    }
}

impl YearFromInput for &String {
    fn parse_year(self) -> Result<Year> {
        self.as_str().parse_year()
    }
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
    /// 
    /// # Examples
    /// 
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// 
    /// let years = Year::all_years();
    /// assert_eq!(years.len(), 201);
    /// assert_eq!(years[0].year, 1900);
    /// assert_eq!(years[200].year, 2100);
    /// ```
    pub fn all_years() -> &'static [Year] {
        &YEARS_ORDERED
    }
    
    /// Parse year from any valid representation
    ///
    /// This method attempts to parse the input using all available parsing methods:
    /// - Number parsing (for i32 values 1900-2100)
    /// - 4-digit string parsing (for strings like "2023", "1999")
    /// - 2-digit string parsing (for strings like "23", "99" with pivot logic)
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// // Parse from number
    /// let year = Year::from(2023i32).unwrap();
    /// assert_eq!(year.year, 2023);
    ///
    /// // Parse from 4-digit string
    /// let year = Year::from("2024").unwrap();
    /// assert_eq!(year.year, 2024);
    ///
    /// // Parse from 2-digit string (with pivot logic)
    /// let year = Year::from("23").unwrap();
    /// assert_eq!(year.year, 2023); // 00-49 = 2000-2049
    ///
    /// let year = Year::from("99").unwrap();
    /// assert_eq!(year.year, 1999); // 50-99 = 1950-1999
    ///
    /// // Invalid cases
    /// assert!(Year::from("invalid").is_err());
    /// assert!(Year::from("1800").is_err());
    /// assert!(Year::from(1800i32).is_err());
    /// ```
    pub fn from<T>(input: T) -> Result<Year>
    where
        T: YearFromInput,
    {
        input.parse_year()
    }
    
    /// Find year by number (1900-2100)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// 
    /// let year = Year::from_number(2023).unwrap();
    /// assert_eq!(year.year, 2023);
    /// assert_eq!(year.text_4d, "2023");
    /// assert_eq!(year.text_2d, "23");
    /// 
    /// // Invalid cases
    /// assert!(Year::from_number(1800).is_err());
    /// assert!(Year::from_number(2200).is_err());
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// // 00-49 maps to 2000-2049
    /// let year = Year::from_2digit_number(23).unwrap();
    /// assert_eq!(year.year, 2023);
    ///
    /// // 50-99 maps to 1950-1999
    /// let year = Year::from_2digit_number(99).unwrap();
    /// assert_eq!(year.year, 1999);
    ///
    /// // Invalid cases
    /// assert!(Year::from_2digit_number(100).is_err());
    /// assert!(Year::from_2digit_number(-1).is_err());
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let next = year.next().unwrap();
    /// assert_eq!(next.year, 2024);
    ///
    /// // Test boundary
    /// let year = Year::from_number(2100).unwrap();
    /// assert!(year.next().is_err()); // Beyond max year
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let prev = year.previous().unwrap();
    /// assert_eq!(prev.year, 2022);
    ///
    /// // Test boundary
    /// let year = Year::from_number(1900).unwrap();
    /// assert!(year.previous().is_err()); // Below min year
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2020).unwrap();
    /// let future = year.add_years(5).unwrap();
    /// assert_eq!(future.year, 2025);
    ///
    /// // Test overflow
    /// let year = Year::from_number(2095).unwrap();
    /// assert!(year.add_years(10).is_err()); // Would exceed max year
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2025).unwrap();
    /// let past = year.subtract_years(5).unwrap();
    /// assert_eq!(past.year, 2020);
    ///
    /// // Test underflow
    /// let year = Year::from_number(1905).unwrap();
    /// assert!(year.subtract_years(10).is_err()); // Would be below min year
    /// ```
    pub fn subtract_years(&self, years: i32) -> Result<Year> {
        self.add_years(-years)
    }
    
    /// Check if this year comes before another chronologically
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year1 = Year::from_number(2020).unwrap();
    /// let year2 = Year::from_number(2025).unwrap();
    ///
    /// assert!(year1.is_before(&year2));
    /// assert!(!year2.is_before(&year1));
    /// assert!(!year1.is_before(&year1));
    /// ```
    pub fn is_before(&self, other: &Year) -> bool {
        self.year < other.year
    }
    
    /// Check if this year comes after another chronologically
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year1 = Year::from_number(2020).unwrap();
    /// let year2 = Year::from_number(2025).unwrap();
    ///
    /// assert!(year2.is_after(&year1));
    /// assert!(!year1.is_after(&year2));
    /// assert!(!year1.is_after(&year1));
    /// ```
    pub fn is_after(&self, other: &Year) -> bool {
        self.year > other.year
    }
    
    /// Get the number of years between this year and another
    /// (positive = forward, negative = backward)
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year1 = Year::from_number(2020).unwrap();
    /// let year2 = Year::from_number(2025).unwrap();
    ///
    /// assert_eq!(year1.years_until(&year2), 5);
    /// assert_eq!(year2.years_until(&year1), -5);
    /// assert_eq!(year1.years_until(&year1), 0);
    /// ```
    pub fn years_until(&self, other: &Year) -> i32 {
        other.year - self.year
    }
    
    /// Get the number of years since another year
    /// (positive = this year is later, negative = this year is earlier)
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year1 = Year::from_number(2020).unwrap();
    /// let year2 = Year::from_number(2025).unwrap();
    ///
    /// assert_eq!(year2.years_since(&year1), 5);
    /// assert_eq!(year1.years_since(&year2), -5);
    /// assert_eq!(year1.years_since(&year1), 0);
    /// ```
    pub fn years_since(&self, other: &Year) -> i32 {
        self.year - other.year
    }
    
    /// Convert to 2-digit string format ("00", "01", etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// assert_eq!(year.to_2digit_string(), "23");
    ///
    /// let year = Year::from_number(2005).unwrap();
    /// assert_eq!(year.to_2digit_string(), "05");
    /// ```
    pub fn to_2digit_string(&self) -> String {
        self.text_2d.to_string()
    }
    
    /// Convert to 4-digit string format ("2023", "1999", etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// assert_eq!(year.to_4digit_string(), "2023");
    ///
    /// let year = Year::from_number(1999).unwrap();
    /// assert_eq!(year.to_4digit_string(), "1999");
    /// ```
    pub fn to_4digit_string(&self) -> String {
        self.text_4d.to_string()
    }
    
    /// Convert to year number (1900-2100)
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// assert_eq!(year.to_number(), 2023);
    ///
    /// let year = Year::from_2digit_number(99).unwrap();
    /// assert_eq!(year.to_number(), 1999);
    /// ```
    pub fn to_number(&self) -> i32 {
        self.year
    }
    
    /// Get 2-digit text representation ("00", "01", etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// assert_eq!(year.to_2digit_text(), "23");
    ///
    /// let year = Year::from_number(2005).unwrap();
    /// assert_eq!(year.to_2digit_text(), "05");
    /// ```
    pub fn to_2digit_text(&self) -> &'static str {
        self.text_2d
    }
    
    /// Get 4-digit text representation ("2023", "1999", etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// assert_eq!(year.to_4digit_text(), "2023");
    ///
    /// let year = Year::from_number(1999).unwrap();
    /// assert_eq!(year.to_4digit_text(), "1999");
    /// ```
    pub fn to_4digit_text(&self) -> &'static str {
        self.text_4d
    }
    
    /// Check if this is a leap year
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2024).unwrap();
    /// assert!(year.is_leap_year());
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// assert!(!year.is_leap_year());
    ///
    /// let year = Year::from_number(2000).unwrap();
    /// assert!(year.is_leap_year()); // Divisible by 400
    ///
    /// let year = Year::from_number(1900).unwrap();
    /// assert!(!year.is_leap_year()); // Divisible by 100 but not 400
    /// ```
    pub fn is_leap_year(&self) -> bool {
        self.is_leap
    }
    
    /// Get the number of days in this year (365 or 366)
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2024).unwrap();
    /// assert_eq!(year.days_in_year(), 366); // Leap year
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// assert_eq!(year.days_in_year(), 365); // Regular year
    /// ```
    pub fn days_in_year(&self) -> u16 {
        if self.is_leap {
            366
        } else {
            365
        }
    }
    
    /// Get the number of days in a specific month of this year
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::{Year, Month};
    ///
    /// let year = Year::from_number(2024).unwrap(); // Leap year
    /// let feb = Month::from_number(2).unwrap();
    /// assert_eq!(year.days_in_month(&feb), 29); // February in leap year
    ///
    /// let year = Year::from_number(2023).unwrap(); // Regular year
    /// assert_eq!(year.days_in_month(&feb), 28); // February in regular year
    ///
    /// let jan = Month::from_number(1).unwrap();
    /// assert_eq!(year.days_in_month(&jan), 31); // January always has 31 days
    /// ```
    pub fn days_in_month(&self, month: &Month) -> u8 {
        match month.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap { 29 } else { 28 },
            _ => unreachable!("Invalid month number"),
        }
    }
    
    /// Get the first day of the year (January 1st)
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// use chrono::NaiveDate;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let start = year.year_start().unwrap();
    /// assert_eq!(start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    /// ```
    pub fn year_start(&self) -> Result<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, 1, 1)
            .ok_or_else(|| UtilsError::Year(
                YearError::invalid_date(self.year, "Cannot create January 1st date")
            ).into())
    }
    
    /// Get the last day of the year (December 31st)
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// use chrono::NaiveDate;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let end = year.year_end().unwrap();
    /// assert_eq!(end, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    /// ```
    pub fn year_end(&self) -> Result<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, 12, 31)
            .ok_or_else(|| UtilsError::Year(
                YearError::invalid_date(self.year, "Cannot create December 31st date")
            ).into())
    }
    
    /// Extract year from a NaiveDate
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
    /// let year = Year::from_naive_date(&date).unwrap();
    /// assert_eq!(year.year, 2023);
    /// ```
    pub fn from_naive_date(date: &NaiveDate) -> Result<Year> {
        let year_num = date.year();
        Self::from_number(year_num)
    }
    
    /// Extract year from a DateTime
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// use chrono::{DateTime, Utc, NaiveDate};
    ///
    /// let datetime = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap()
    ///     .and_hms_opt(14, 30, 0).unwrap()
    ///     .and_utc();
    /// let year = Year::from_datetime(&datetime).unwrap();
    /// assert_eq!(year.year, 2023);
    /// ```
    pub fn from_datetime<Tz: TimeZone>(datetime: &DateTime<Tz>) -> Result<Year> {
        let year_num = datetime.year();
        Self::from_number(year_num)
    }
    
    /// Create a NaiveDate from this year, month, and day
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::{Year, Month};
    /// use chrono::NaiveDate;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let month = Month::from_number(6).unwrap();
    /// let date = year.to_naive_date(&month, 15).unwrap();
    /// assert_eq!(date, NaiveDate::from_ymd_opt(2023, 6, 15).unwrap());
    ///
    /// // Invalid date
    /// let feb = Month::from_number(2).unwrap();
    /// assert!(year.to_naive_date(&feb, 30).is_err()); // February 30th doesn't exist
    /// ```
    pub fn to_naive_date(&self, month: &Month, day: u32) -> Result<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, month.month as u32, day)
            .ok_or_else(|| UtilsError::Year(
                YearError::invalid_date(self.year, format!("Cannot create date {}-{:02}-{:02}", self.year, month.month, day))
            ).into())
    }
    
    /// Create a DateTime<Utc> from this year, month, day, hour, minute, and second
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::{Year, Month};
    /// use chrono::{DateTime, Utc, NaiveDate};
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let month = Month::from_number(6).unwrap();
    /// let datetime = year.to_datetime_utc(&month, 15, 14, 30, 0).unwrap();
    ///
    /// let expected = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap()
    ///     .and_hms_opt(14, 30, 0).unwrap()
    ///     .and_utc();
    /// assert_eq!(datetime, expected);
    /// ```
    pub fn to_datetime_utc(&self, month: &Month, day: u32, hour: u32, min: u32, sec: u32) -> Result<DateTime<chrono::Utc>> {
        let naive_date = self.to_naive_date(month, day)?;
        let naive_time = chrono::NaiveTime::from_hms_opt(hour, min, sec)
            .ok_or_else(|| UtilsError::Year(
                YearError::invalid_date(self.year, format!("Cannot create time {:02}:{:02}:{:02}", hour, min, sec))
            ))?;
        
        Ok(naive_date.and_time(naive_time).and_utc())
    }
    
    /// Create a DateTime with a specific timezone
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::{Year, Month};
    /// use chrono::{DateTime, Local, NaiveDate};
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let month = Month::from_number(6).unwrap();
    /// let datetime = year.to_datetime_with_timezone(&month, 15, 14, 30, 0, &Local).unwrap();
    ///
    /// let expected = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap()
    ///     .and_hms_opt(14, 30, 0).unwrap();
    /// assert_eq!(datetime.naive_local(), expected);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// use chrono::{DateTime, Utc, NaiveDate};
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let start = year.year_start_in_timezone(&Utc).unwrap();
    ///
    /// let expected = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
    ///     .and_hms_opt(0, 0, 0).unwrap()
    ///     .and_utc();
    /// assert_eq!(start, expected);
    /// ```
    pub fn year_start_in_timezone<Tz: TimeZone>(&self, tz: &Tz) -> Result<DateTime<Tz>> {
        let jan = Month::from_number(1).map_err(|e| UtilsError::Year(
            YearError::chrono_conversion(format!("Cannot get January: {}", e))
        ))?;
        self.to_datetime_with_timezone(&jan, 1, 0, 0, 0, tz)
    }
    
    /// Get the end of the year in a specific timezone
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// use chrono::{DateTime, Utc, NaiveDate};
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let end = year.year_end_in_timezone(&Utc).unwrap();
    ///
    /// let expected = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()
    ///     .and_hms_opt(23, 59, 59).unwrap()
    ///     .and_utc();
    /// assert_eq!(end, expected);
    /// ```
    pub fn year_end_in_timezone<Tz: TimeZone>(&self, tz: &Tz) -> Result<DateTime<Tz>> {
        let dec = Month::from_number(12).map_err(|e| UtilsError::Year(
            YearError::chrono_conversion(format!("Cannot get December: {}", e))
        ))?;
        self.to_datetime_with_timezone(&dec, 31, 23, 59, 59, tz)
    }
    
    /// Get the first day of a specific quarter
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// use chrono::NaiveDate;
    ///
    /// let year = Year::from_number(2023).unwrap();
    ///
    /// let q1_start = year.quarter_start(1).unwrap();
    /// assert_eq!(q1_start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    ///
    /// let q2_start = year.quarter_start(2).unwrap();
    /// assert_eq!(q2_start, NaiveDate::from_ymd_opt(2023, 4, 1).unwrap());
    ///
    /// let q3_start = year.quarter_start(3).unwrap();
    /// assert_eq!(q3_start, NaiveDate::from_ymd_opt(2023, 7, 1).unwrap());
    ///
    /// let q4_start = year.quarter_start(4).unwrap();
    /// assert_eq!(q4_start, NaiveDate::from_ymd_opt(2023, 10, 1).unwrap());
    ///
    /// // Invalid quarter
    /// assert!(year.quarter_start(5).is_err());
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// use chrono::NaiveDate;
    ///
    /// let year = Year::from_number(2023).unwrap();
    ///
    /// let q1_end = year.quarter_end(1).unwrap();
    /// assert_eq!(q1_end, NaiveDate::from_ymd_opt(2023, 3, 31).unwrap());
    ///
    /// let q2_end = year.quarter_end(2).unwrap();
    /// assert_eq!(q2_end, NaiveDate::from_ymd_opt(2023, 6, 30).unwrap());
    ///
    /// let q3_end = year.quarter_end(3).unwrap();
    /// assert_eq!(q3_end, NaiveDate::from_ymd_opt(2023, 9, 30).unwrap());
    ///
    /// let q4_end = year.quarter_end(4).unwrap();
    /// assert_eq!(q4_end, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::{Year, Month};
    /// use chrono::NaiveDate;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let june = Month::from_number(6).unwrap();
    ///
    /// let month_start = year.month_start(&june).unwrap();
    /// assert_eq!(month_start, NaiveDate::from_ymd_opt(2023, 6, 1).unwrap());
    /// ```
    pub fn month_start(&self, month: &Month) -> Result<NaiveDate> {
        self.to_naive_date(month, 1)
    }
    
    /// Get the last day of a specific month in this year
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::{Year, Month};
    /// use chrono::NaiveDate;
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let june = Month::from_number(6).unwrap();
    /// let feb = Month::from_number(2).unwrap();
    ///
    /// let june_end = year.month_end(&june).unwrap();
    /// assert_eq!(june_end, NaiveDate::from_ymd_opt(2023, 6, 30).unwrap());
    ///
    /// let feb_end = year.month_end(&feb).unwrap();
    /// assert_eq!(feb_end, NaiveDate::from_ymd_opt(2023, 2, 28).unwrap()); // Non-leap year
    ///
    /// // Test leap year
    /// let leap_year = Year::from_number(2024).unwrap();
    /// let leap_feb_end = leap_year.month_end(&feb).unwrap();
    /// assert_eq!(leap_feb_end, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()); // Leap year
    /// ```
    pub fn month_end(&self, month: &Month) -> Result<NaiveDate> {
        let last_day = self.days_in_month(month);
        self.to_naive_date(month, last_day as u32)
    }
    
    /// Check if a specific date (month and day) is valid for this year
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::{Year, Month};
    ///
    /// let year = Year::from_number(2023).unwrap(); // Non-leap year
    /// let leap_year = Year::from_number(2024).unwrap(); // Leap year
    /// let feb = Month::from_number(2).unwrap();
    /// let june = Month::from_number(6).unwrap();
    ///
    /// // Valid dates
    /// assert!(year.is_valid_date(&feb, 28));
    /// assert!(year.is_valid_date(&june, 30));
    /// assert!(leap_year.is_valid_date(&feb, 29)); // Valid in leap year
    ///
    /// // Invalid dates
    /// assert!(!year.is_valid_date(&feb, 29)); // Invalid in non-leap year
    /// assert!(!year.is_valid_date(&june, 31)); // June has only 30 days
    /// assert!(!year.is_valid_date(&feb, 0)); // Day 0 doesn't exist
    /// ```
    pub fn is_valid_date(&self, month: &Month, day: u32) -> bool {
        if day == 0 {
            return false;
        }
        let max_days = self.days_in_month(month) as u32;
        day <= max_days
    }
    
    /// Get all months of this year with their start and end dates
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// use chrono::{Datelike, NaiveDate};
    ///
    /// let year = Year::from_number(2023).unwrap();
    /// let month_ranges = year.month_ranges().unwrap();
    ///
    /// assert_eq!(month_ranges.len(), 12);
    ///
    /// // Check January
    /// let (jan_month, jan_start, jan_end) = &month_ranges[0];
    /// assert_eq!(jan_month.month, 1);
    /// assert_eq!(jan_start.day(), 1);
    /// assert_eq!(jan_end.day(), 31);
    ///
    /// // Check February (non-leap year)
    /// let (feb_month, feb_start, feb_end) = &month_ranges[1];
    /// assert_eq!(feb_month.month, 2);
    /// assert_eq!(feb_start.day(), 1);
    /// assert_eq!(feb_end.day(), 28); // Non-leap year
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::{Year, Month};
    ///
    /// let year = Year::from_number(2023).unwrap();
    ///
    /// let jan = Month::from_number(1).unwrap();
    /// let apr = Month::from_number(4).unwrap();
    /// let jul = Month::from_number(7).unwrap();
    /// let oct = Month::from_number(10).unwrap();
    ///
    /// assert_eq!(year.get_quarter(&jan), 1);
    /// assert_eq!(year.get_quarter(&apr), 2);
    /// assert_eq!(year.get_quarter(&jul), 3);
    /// assert_eq!(year.get_quarter(&oct), 4);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    ///
    /// let year = Year::from_number(2023).unwrap();
    ///
    /// let q1_months = year.get_quarter_months(1).unwrap();
    /// assert_eq!(q1_months.len(), 3);
    /// assert_eq!(q1_months[0].month, 1); // January
    /// assert_eq!(q1_months[1].month, 2); // February
    /// assert_eq!(q1_months[2].month, 3); // March
    ///
    /// let q4_months = year.get_quarter_months(4).unwrap();
    /// assert_eq!(q4_months.len(), 3);
    /// assert_eq!(q4_months[0].month, 10); // October
    /// assert_eq!(q4_months[1].month, 11); // November
    /// assert_eq!(q4_months[2].month, 12); // December
    ///
    /// // Invalid quarter
    /// assert!(year.get_quarter_months(5).is_err());
    /// ```
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
    /// 
    /// # Examples
    /// 
    /// ```
    /// use arrow_sus_shared::utils::time::Year;
    /// 
    /// // Works with i32
    /// assert!(Year::is_valid(2023i32));
    /// assert!(!Year::is_valid(1800i32));
    /// 
    /// // Works with &str - various formats
    /// assert!(Year::is_valid("2023"));
    /// assert!(Year::is_valid("23")); // 2-digit
    /// 
    /// // Works with String
    /// assert!(Year::is_valid(String::from("2024")));
    /// 
    /// // Invalid cases
    /// assert!(!Year::is_valid("invalid"));
    /// assert!(!Year::is_valid("1800"));
    /// assert!(!Year::is_valid("2200"));
    /// ```
    pub fn is_valid<T: YearValidatable>(input: T) -> bool {
        input.is_valid_year()
    }
}
