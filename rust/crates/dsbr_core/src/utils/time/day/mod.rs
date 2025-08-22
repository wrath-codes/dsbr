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
    (1..=31).for_each(|i| {
        days.insert(Day::new_unchecked(i));
    });
    days
});

pub static DAYS_ORDERED: LazyLock<[Day; 31]> = LazyLock::new(|| {
    (1..=31)
        .map(Day::new_unchecked)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
});

impl Day {
    // Static lookup tables for efficient text generation
    const DAY_TEXTS: [&'static str; 31] = [
        "01", "02", "03", "04", "05", "06", "07", "08", "09", "10",
        "11", "12", "13", "14", "15", "16", "17", "18", "19", "20",
        "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31"
    ];
    
    const ORDINAL_EN: [&'static str; 31] = [
        "1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th",
        "11th", "12th", "13th", "14th", "15th", "16th", "17th", "18th", "19th", "20th",
        "21st", "22nd", "23rd", "24th", "25th", "26th", "27th", "28th", "29th", "30th", "31st"
    ];
    
    const ORDINAL_PTBR: [&'static str; 31] = [
        "1º", "2º", "3º", "4º", "5º", "6º", "7º", "8º", "9º", "10º",
        "11º", "12º", "13º", "14º", "15º", "16º", "17º", "18º", "19º", "20º",
        "21º", "22º", "23º", "24º", "25º", "26º", "27º", "28º", "29º", "30º", "31º"
    ];

    /// Create a new Day without validation (internal use only)
    fn new_unchecked(day: u8) -> Self {
        let index = (day - 1) as usize;
        
        Self {
            day,
            text: Self::DAY_TEXTS[index],
            ordinal_en: Self::ORDINAL_EN[index],
            ordinal_ptbr: Self::ORDINAL_PTBR[index],
        }
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
        match day {
            1..=31 => {
                let index = (day - 1) as usize;
                Ok(Self::all_days()[index])
            }
            _ => Err(UtilsError::Day(
                DayError::invalid_day(day)
            ).into()),
        }
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
        match self.day {
            d if d <= max_days => true,
            _ => false,
        }
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
    
    // Static lookup tables for weekday names
    const WEEKDAY_NAMES_EN: [&'static str; 7] = [
        "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"
    ];
    
    const WEEKDAY_NAMES_PTBR: [&'static str; 7] = [
        "Segunda-feira", "Terça-feira", "Quarta-feira", "Quinta-feira",
        "Sexta-feira", "Sábado", "Domingo"
    ];
    
    const WEEKDAY_SHORT_EN: [&'static str; 7] = [
        "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"
    ];
    
    const WEEKDAY_SHORT_PTBR: [&'static str; 7] = [
        "Seg", "Ter", "Qua", "Qui", "Sex", "Sáb", "Dom"
    ];

    /// Get localized weekday name
    pub fn to_weekday_name_en(&self, month: &Month, year: &Year) -> Result<&'static str> {
        let weekday = self.to_weekday(month, year)?;
        Ok(Self::WEEKDAY_NAMES_EN[weekday.num_days_from_monday() as usize])
    }
    
    pub fn to_weekday_name_ptbr(&self, month: &Month, year: &Year) -> Result<&'static str> {
        let weekday = self.to_weekday(month, year)?;
        Ok(Self::WEEKDAY_NAMES_PTBR[weekday.num_days_from_monday() as usize])
    }
    
    pub fn to_weekday_short_en(&self, month: &Month, year: &Year) -> Result<&'static str> {
        let weekday = self.to_weekday(month, year)?;
        Ok(Self::WEEKDAY_SHORT_EN[weekday.num_days_from_monday() as usize])
    }
    
    pub fn to_weekday_short_ptbr(&self, month: &Month, year: &Year) -> Result<&'static str> {
        let weekday = self.to_weekday(month, year)?;
        Ok(Self::WEEKDAY_SHORT_PTBR[weekday.num_days_from_monday() as usize])
    }
    
    /// Navigation methods
    pub fn next(&self) -> Option<Day> {
        match self.day {
            31 => None,
            d => Some(Self::all_days()[d as usize]), // day + 1, but 0-indexed
        }
    }
    
    pub fn previous(&self) -> Option<Day> {
        match self.day {
            1 => None,
            d => Some(Self::all_days()[(d - 2) as usize]), // day - 1, but 0-indexed
        }
    }
    
    /// Context-aware navigation
    pub fn next_in_month(&self, month: &Month, year: &Year) -> Option<Day> {
        let next_day = self.next()?;
        match next_day.is_valid_for_month(month, year) {
            true => Some(next_day),
            false => None,
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
        match input.parse::<u8>() {
            Ok(num) => Self::is_valid_day_number(num),
            Err(_) => false,
        }
    }
}