use dashmap::DashSet;
use std::sync::LazyLock;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, DateTime, Weekday, Datelike, TimeZone};
use crate::core::Result;
use crate::utils::{UtilsError};
use crate::utils::time::{Month, Year};

pub mod error;
pub mod traits;
#[cfg(test)]
mod tests;

pub use error::DayError;
pub use traits::{DayValidatable, DayFromInput};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct Day {
    pub day: u8,
    pub text: &'static str,
    pub ordinal_en: &'static str,
    pub ordinal_ptbr: &'static str,
}

// Static collections following existing pattern
pub static DAYS: LazyLock<DashSet<Day>> = LazyLock::new(|| {
    let days = DashSet::with_capacity(31);
    for i in 1..=31 {
        days.insert(Day::new_unchecked(i));
    }
    days
});

pub static DAYS_ORDERED: LazyLock<[Day; 31]> = LazyLock::new(|| {
    let mut days_vec: Vec<Day> = DAYS.iter().map(|day_ref| *day_ref).collect();
    days_vec.sort_by_key(|day| day.day);
    days_vec.try_into().unwrap()
});

impl Day {
    /// Create a new Day without validation (internal use only)
    fn new_unchecked(day: u8) -> Self {
        let text = match day {
            1 => "01", 2 => "02", 3 => "03", 4 => "04", 5 => "05",
            6 => "06", 7 => "07", 8 => "08", 9 => "09", 10 => "10",
            11 => "11", 12 => "12", 13 => "13", 14 => "14", 15 => "15",
            16 => "16", 17 => "17", 18 => "18", 19 => "19", 20 => "20",
            21 => "21", 22 => "22", 23 => "23", 24 => "24", 25 => "25",
            26 => "26", 27 => "27", 28 => "28", 29 => "29", 30 => "30",
            31 => "31",
            _ => unreachable!("Invalid day number"),
        };
        
        let ordinal_en = match day {
            1 | 21 | 31 => match day { 1 => "1st", 21 => "21st", 31 => "31st", _ => unreachable!() },
            2 | 22 => match day { 2 => "2nd", 22 => "22nd", _ => unreachable!() },
            3 | 23 => match day { 3 => "3rd", 23 => "23rd", _ => unreachable!() },
            _ => match day {
                4 => "4th", 5 => "5th", 6 => "6th", 7 => "7th", 8 => "8th", 9 => "9th", 10 => "10th",
                11 => "11th", 12 => "12th", 13 => "13th", 14 => "14th", 15 => "15th", 16 => "16th",
                17 => "17th", 18 => "18th", 19 => "19th", 20 => "20th", 24 => "24th", 25 => "25th",
                26 => "26th", 27 => "27th", 28 => "28th", 29 => "29th", 30 => "30th",
                _ => unreachable!(),
            }
        };
        
        let ordinal_ptbr = match day {
            1 => "1º", 2 => "2º", 3 => "3º", 4 => "4º", 5 => "5º",
            6 => "6º", 7 => "7º", 8 => "8º", 9 => "9º", 10 => "10º",
            11 => "11º", 12 => "12º", 13 => "13º", 14 => "14º", 15 => "15º",
            16 => "16º", 17 => "17º", 18 => "18º", 19 => "19º", 20 => "20º",
            21 => "21º", 22 => "22º", 23 => "23º", 24 => "24º", 25 => "25º",
            26 => "26º", 27 => "27º", 28 => "28º", 29 => "29º", 30 => "30º",
            31 => "31º",
            _ => unreachable!(),
        };
        
        Self { day, text, ordinal_en, ordinal_ptbr }
    }
    
    /// Returns all days in order (1 to 31)
    pub fn all_days() -> &'static [Day; 31] {
        &DAYS_ORDERED
    }
    
    /// Parse day from any valid representation
    pub fn from<T>(input: T) -> Result<Day>
    where
        T: DayFromInput,
    {
        input.parse_day()
    }
    
    /// Find day by number (1-31)
    pub fn from_number(day: u8) -> Result<Day> {
        if day < 1 || day > 31 {
            return Err(UtilsError::Day(
                DayError::invalid_day(day)
            ).into());
        }
        
        // Direct indexing - O(1) lookup
        let index = (day - 1) as usize;
        Ok(Self::all_days()[index])
    }
    
    /// Check if this day is valid for a specific month and year
    pub fn is_valid_for_month(&self, month: &Month, year: &Year) -> bool {
        year.is_valid_date(month, self.day as u32)
    }
    
    /// Check if this day is valid for a specific month (non-leap year logic)
    pub fn is_valid_for_month_simple(&self, month: &Month) -> bool {
        let max_days = match month.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => 28, // Non-leap year February
            _ => unreachable!("Invalid month number"),
        };
        self.day <= max_days
    }
    
    /// Create a NaiveDate from this day with month and year
    pub fn to_naive_date(&self, month: &Month, year: &Year) -> Result<NaiveDate> {
        year.to_naive_date(month, self.day as u32)
    }
    
    /// Get the day of week for this day in a specific month/year
    pub fn to_weekday(&self, month: &Month, year: &Year) -> Result<Weekday> {
        let date = self.to_naive_date(month, year)?;
        Ok(date.weekday())
    }
    
    /// Get localized weekday name
    pub fn to_weekday_name_en(&self, month: &Month, year: &Year) -> Result<&'static str> {
        let weekday = self.to_weekday(month, year)?;
        Ok(match weekday {
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday", 
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturday",
            Weekday::Sun => "Sunday",
        })
    }
    
    pub fn to_weekday_name_ptbr(&self, month: &Month, year: &Year) -> Result<&'static str> {
        let weekday = self.to_weekday(month, year)?;
        Ok(match weekday {
            Weekday::Mon => "Segunda-feira",
            Weekday::Tue => "Terça-feira",
            Weekday::Wed => "Quarta-feira", 
            Weekday::Thu => "Quinta-feira",
            Weekday::Fri => "Sexta-feira",
            Weekday::Sat => "Sábado",
            Weekday::Sun => "Domingo",
        })
    }
    
    pub fn to_weekday_short_en(&self, month: &Month, year: &Year) -> Result<&'static str> {
        let weekday = self.to_weekday(month, year)?;
        Ok(match weekday {
            Weekday::Mon => "Mon",
            Weekday::Tue => "Tue",
            Weekday::Wed => "Wed", 
            Weekday::Thu => "Thu",
            Weekday::Fri => "Fri",
            Weekday::Sat => "Sat",
            Weekday::Sun => "Sun",
        })
    }
    
    pub fn to_weekday_short_ptbr(&self, month: &Month, year: &Year) -> Result<&'static str> {
        let weekday = self.to_weekday(month, year)?;
        Ok(match weekday {
            Weekday::Mon => "Seg",
            Weekday::Tue => "Ter",
            Weekday::Wed => "Qua",
            Weekday::Thu => "Qui", 
            Weekday::Fri => "Sex",
            Weekday::Sat => "Sáb",
            Weekday::Sun => "Dom",
        })
    }
    
    /// Navigation methods
    pub fn next(&self) -> Option<Day> {
        if self.day >= 31 {
            None
        } else {
            Some(Self::all_days()[self.day as usize]) // day + 1, but 0-indexed
        }
    }
    
    pub fn previous(&self) -> Option<Day> {
        if self.day <= 1 {
            None
        } else {
            Some(Self::all_days()[(self.day - 2) as usize]) // day - 1, but 0-indexed
        }
    }
    
    /// Context-aware navigation
    pub fn next_in_month(&self, month: &Month, year: &Year) -> Option<Day> {
        let next_day = self.next()?;
        if next_day.is_valid_for_month(month, year) {
            Some(next_day)
        } else {
            None
        }
    }
    
    pub fn previous_in_month(&self, _month: &Month, _year: &Year) -> Option<Day> {
        self.previous() // Always valid if previous exists
    }
    
    /// Extract day from NaiveDate
    pub fn from_naive_date(date: &NaiveDate) -> Result<Day> {
        let day_num = date.day() as u8;
        Self::from_number(day_num)
    }
    
    /// Extract day from DateTime<Utc>
    pub fn from_datetime_utc(datetime: &DateTime<chrono::Utc>) -> Result<Day> {
        let day_num = datetime.day() as u8;
        Self::from_number(day_num)
    }
    
    /// Extract day from any DateTime with timezone
    pub fn from_datetime<Tz: TimeZone>(datetime: &DateTime<Tz>) -> Result<Day> {
        let day_num = datetime.day() as u8;
        Self::from_number(day_num)
    }
    
    /// Create a DateTime<Utc> from this day with month, year, and time components
    pub fn to_datetime_utc(&self, month: &Month, year: &Year, hour: u32, min: u32, sec: u32) -> Result<DateTime<chrono::Utc>> {
        year.to_datetime_utc(month, self.day as u32, hour, min, sec)
    }
    
    /// Create a DateTime with specific timezone from this day with month, year, and time components
    pub fn to_datetime_with_timezone<Tz: TimeZone>(&self, month: &Month, year: &Year, hour: u32, min: u32, sec: u32, tz: &Tz) -> Result<DateTime<Tz>> {
        year.to_datetime_with_timezone(month, self.day as u32, hour, min, sec, tz)
    }
    
    /// Create a DateTime<Utc> at start of day (00:00:00)
    pub fn to_datetime_start_of_day_utc(&self, month: &Month, year: &Year) -> Result<DateTime<chrono::Utc>> {
        self.to_datetime_utc(month, year, 0, 0, 0)
    }
    
    /// Create a DateTime<Utc> at end of day (23:59:59)
    pub fn to_datetime_end_of_day_utc(&self, month: &Month, year: &Year) -> Result<DateTime<chrono::Utc>> {
        self.to_datetime_utc(month, year, 23, 59, 59)
    }
    
    /// Create a DateTime with timezone at start of day (00:00:00)
    pub fn to_datetime_start_of_day_with_timezone<Tz: TimeZone>(&self, month: &Month, year: &Year, tz: &Tz) -> Result<DateTime<Tz>> {
        self.to_datetime_with_timezone(month, year, 0, 0, 0, tz)
    }
    
    /// Create a DateTime with timezone at end of day (23:59:59)
    pub fn to_datetime_end_of_day_with_timezone<Tz: TimeZone>(&self, month: &Month, year: &Year, tz: &Tz) -> Result<DateTime<Tz>> {
        self.to_datetime_with_timezone(month, year, 23, 59, 59, tz)
    }
    
    /// Conversion methods
    pub fn to_number(&self) -> u8 {
        self.day
    }
    
    pub fn to_text(&self) -> &'static str {
        self.text
    }
    
    pub fn to_zero_padded_string(&self) -> String {
        self.text.to_string()
    }
    
    pub fn to_number_string(&self) -> String {
        self.day.to_string()
    }
    
    pub fn to_ordinal_en(&self) -> &'static str {
        self.ordinal_en
    }
    
    pub fn to_ordinal_ptbr(&self) -> &'static str {
        self.ordinal_ptbr
    }
    
    /// Validation methods
    pub fn is_valid<T: DayValidatable>(input: T) -> bool {
        input.is_valid_day()
    }
    
    pub fn is_valid_day_number(day: u8) -> bool {
        day >= 1 && day <= 31
    }
    
    pub fn is_valid_day_string(input: &str) -> bool {
        if let Ok(num) = input.parse::<u8>() {
            Self::is_valid_day_number(num)
        } else {
            false
        }
    }
}