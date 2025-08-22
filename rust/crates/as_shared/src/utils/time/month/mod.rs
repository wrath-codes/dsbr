use dashmap::{DashSet};
use std::sync::LazyLock;
use serde::{Serialize, Deserialize};
use crate::core::{Result};
use crate::utils::{UtilsError};

pub mod error;
pub mod traits;
#[cfg(test)]
mod tests;

pub use error::MonthError;
pub use traits::{MonthValidatable, MonthFromInput};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct Month {
    pub month: u8,
    pub text: &'static str,
    pub name_ptbr: &'static str,
    pub name_short: &'static str,
    pub name_en: &'static str,
}

impl Month {
    // Static lookup tables for efficient month data
    const MONTH_TEXTS: [&'static str; 12] = [
        "01", "02", "03", "04", "05", "06",
        "07", "08", "09", "10", "11", "12"
    ];
    
    const MONTH_NAMES_PTBR: [&'static str; 12] = [
        "Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho",
        "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"
    ];
    
    const MONTH_NAMES_SHORT: [&'static str; 12] = [
        "Jan", "Fev", "Mar", "Abr", "Mai", "Jun",
        "Jul", "Ago", "Set", "Out", "Nov", "Dez"
    ];
    
    const MONTH_NAMES_EN: [&'static str; 12] = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];

    /// Create a new Month from number (internal use)
    fn new_unchecked(month: u8) -> Self {
        let index = (month - 1) as usize;
        Self {
            month,
            text: Self::MONTH_TEXTS[index],
            name_ptbr: Self::MONTH_NAMES_PTBR[index],
            name_short: Self::MONTH_NAMES_SHORT[index],
            name_en: Self::MONTH_NAMES_EN[index],
        }
    }
}

pub static MONTHS: LazyLock<DashSet<Month>> = LazyLock::new(|| {
    let months = DashSet::with_capacity(12);
    (1..=12).for_each(|i| {
        months.insert(Month::new_unchecked(i));
    });
    months
});

pub static MONTHS_ORDERED: LazyLock<[Month; 12]> = LazyLock::new(|| {
    (1..=12)
        .map(Month::new_unchecked)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
});

impl Month {
    /// Returns all months in chronological order (January to December)
    pub fn all_months() -> &'static [Month; 12] {
        &MONTHS_ORDERED
    }
   
    /// Get the next month (December wraps to January)
    pub fn next(&self) -> Month {
        let months = Self::all_months();
        let current_index = (self.month - 1) as usize;
        let next_index = (current_index + 1) % 12;
        months[next_index]
    }

    /// Get the previous month (January wraps to December)
    pub fn previous(&self) -> Month {
        let months = Self::all_months();
        let current_index = (self.month - 1) as usize;
        let prev_index = match current_index {
            0 => 11,
            i => i - 1,
        };
        months[prev_index]
    }
    
    /// Check if this month comes before another chronologically
    pub fn is_before(&self, other: &Month) -> bool {
        self.month < other.month
    }

    /// Check if this month comes after another chronologically
    pub fn is_after(&self, other: &Month) -> bool {
        self.month > other.month
    }

    /// Get the number of months between this month and another
    /// (positive = forward, negative = backward)
    pub fn months_until(&self, other: &Month) -> i8 {
        other.month as i8 - self.month as i8
    }

    /// Get the number of months since another month
    /// (positive = this month is later, negative = this month is earlier)
    pub fn months_since(&self, other: &Month) -> i8 {
        self.month as i8 - other.month as i8
    }

    /// Convert to zero-padded string format ("01", "02", etc.)
    pub fn to_zero_padded_string(&self) -> String {
        self.text.to_string()
    }

    /// Convert to number string format ("1", "2", etc.)
    pub fn to_number_string(&self) -> String {
        self.month.to_string()
    }

    /// Convert to month number (1-12)
    pub fn to_number(&self) -> u8 {
        self.month
    }

    /// Convert to zero-padded text representation ("01", "02", etc.)
    pub fn to_text(&self) -> &'static str {
        self.text
    }

    /// Convert to Portuguese name ("Janeiro", "Fevereiro", etc.)
    pub fn to_ptbr(&self) -> &'static str {
        self.name_ptbr
    }

    /// Convert to English name ("January", "February", etc.)
    pub fn to_en(&self) -> &'static str {
        self.name_en
    }

    /// Convert to short/abbreviated name ("Jan", "Feb", etc.)
    pub fn to_short(&self) -> &'static str {
        self.name_short
    }

    /// Parse month from any valid representation
    ///
    /// This method attempts to parse the input using all available parsing methods:
    /// - Number parsing (for u8 values 1-12)
    /// - Text parsing (for zero-padded strings like "01", "02")
    /// - Number string parsing (for strings like "1", "2")
    /// - English name parsing (case-insensitive)
    /// - Portuguese name parsing (case-insensitive)
    /// - Abbreviation parsing (case-insensitive)
    pub fn from<T>(input: T) -> Result<Month>
    where
        T: MonthFromInput,
    {
        input.parse_month()
    }
    
    /// Find month by number (1-12)
    pub fn from_number(month: u8) -> Result<Month> {
        match month {
            1..=12 => {
                let index = (month - 1) as usize;
                Ok(Self::all_months()[index])
            }
            _ => Err(UtilsError::Month(
                MonthError::not_valid_month_number(format!("{}", month))
            ).into()),
        }
    }

    /// Find month by text representation ("01", "02", etc.)
    pub fn from_text(text: &str) -> Result<Month> {
        Self::MONTH_TEXTS
            .iter()
            .position(|&month_text| month_text == text)
            .map(|index| Self::new_unchecked((index + 1) as u8))
            .ok_or_else(|| UtilsError::Month(
                MonthError::not_valid_month_string(text.to_string())
            ).into())
    }

    /// Find month by English name ("January", "February", etc.)
    pub fn from_english_name(name: &str) -> Result<Month> {
        Self::MONTH_NAMES_EN
            .iter()
            .position(|&month_name| month_name.eq_ignore_ascii_case(name))
            .map(|index| Self::new_unchecked((index + 1) as u8))
            .ok_or_else(|| UtilsError::Month(
                MonthError::not_valid_month_english(name.to_string())
            ).into())
    }

    /// Find month by Portuguese name ("Janeiro", "Fevereiro", etc.)
    pub fn from_portuguese_name(name: &str) -> Result<Month> {
        Self::MONTH_NAMES_PTBR
            .iter()
            .position(|&month_name| month_name.to_lowercase() == name.to_lowercase())
            .map(|index| Self::new_unchecked((index + 1) as u8))
            .ok_or_else(|| UtilsError::Month(
                MonthError::not_valid_month_portuguese(name.to_string())
            ).into())
    }

    /// Find month by abbreviation ("Jan", "Feb", etc.)
    pub fn from_abbreviation(abbr: &str) -> Result<Month> {
        Self::MONTH_NAMES_SHORT
            .iter()
            .position(|&month_abbr| month_abbr.eq_ignore_ascii_case(abbr))
            .map(|index| Self::new_unchecked((index + 1) as u8))
            .ok_or_else(|| UtilsError::Month(
                MonthError::not_valid_month_abbreviation(abbr.to_string())
            ).into())
    }
    
    // Private methods made public for trait implementations
    pub fn is_valid_month_number(month: u8) -> bool {
        match month {
            1..=12 => true,
            _ => false,
        }
    }

    pub fn is_valid_month_text(text: &str) -> bool {
        Self::all_months()
            .iter()
            .any(|month| month.text == text)
    }

    pub fn is_valid_english_name(name: &str) -> bool {
        Self::all_months()
            .iter()
            .any(|month| month.name_en.eq_ignore_ascii_case(name))
    }

    pub fn is_valid_portuguese_name(name: &str) -> bool {
        Self::all_months()
            .iter()
            .any(|month| month.name_ptbr.to_lowercase() == name.to_lowercase())
    }

    pub fn is_valid_abbreviation(abbr: &str) -> bool {
        Self::all_months()
            .iter()
            .any(|month| month.name_short.eq_ignore_ascii_case(abbr))
    }

    pub fn is_valid_month_number_string(input: &str) -> bool {
        match input.parse::<u8>() {
            Ok(num) => Self::is_valid_month_number(num),
            Err(_) => false,
        }
    }

    /// Public generic validation method - accepts both u8 and string types
    pub fn is_valid<T: MonthValidatable>(input: T) -> bool {
        input.is_valid_month()
    }
}