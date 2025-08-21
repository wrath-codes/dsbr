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

pub static MONTHS: LazyLock<DashSet<Month>> = LazyLock::new(|| {
    let months = DashSet::with_capacity(12);
    months.insert(Month{month: 1, text: "01", name_ptbr: "Janeiro", name_short: "Jan", name_en: "January"});
    months.insert(Month{month: 2, text: "02", name_ptbr: "Fevereiro", name_short: "Fev", name_en: "February"});
    months.insert(Month{month: 3, text: "03", name_ptbr: "Março", name_short: "Mar", name_en: "March"});
    months.insert(Month{month: 4, text: "04", name_ptbr: "Abril", name_short: "Abr", name_en: "April"});
    months.insert(Month{month: 5, text: "05", name_ptbr: "Maio", name_short: "Mai", name_en: "May"});
    months.insert(Month{month: 6, text: "06", name_ptbr: "Junho", name_short: "Jun", name_en: "June"});
    months.insert(Month{month: 7, text: "07", name_ptbr: "Julho", name_short: "Jul", name_en: "July"});
    months.insert(Month{month: 8, text: "08", name_ptbr: "Agosto", name_short: "Ago", name_en: "August"});
    months.insert(Month{month: 9, text: "09", name_ptbr: "Setembro", name_short: "Set", name_en: "September"});
    months.insert(Month{month: 10, text: "10", name_ptbr: "Outubro", name_short: "Out", name_en: "October"});
    months.insert(Month{month: 11, text: "11", name_ptbr: "Novembro", name_short: "Nov", name_en: "November"});
    months.insert(Month{month: 12, text: "12", name_ptbr: "Dezembro", name_short: "Dez", name_en: "December"});
    months
});

pub static MONTHS_ORDERED: LazyLock<[Month; 12]> = LazyLock::new(|| {
    let mut months_vec: Vec<Month> = MONTHS.iter().map(|month_ref| *month_ref).collect();
    months_vec.sort_by_key(|month| month.month);
    months_vec.try_into().unwrap()
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
        let prev_index = if current_index == 0 { 11 } else { current_index - 1 };
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
        if month < 1 || month > 12 {
            return Err(UtilsError::Month(
                MonthError::not_valid_month_number(format!("{}", month))
            ).into());
        }
        
        // Direct indexing - O(1) lookup
        let index = (month - 1) as usize;
        Ok(Self::all_months()[index])
    }

    /// Find month by text representation ("01", "02", etc.)
    pub fn from_text(text: &str) -> Result<Month> {
        Self::all_months()
            .iter()
            .find(|month| month.text == text)
            .copied()
            .ok_or_else(|| UtilsError::Month(
                MonthError::not_valid_month_string(text.to_string())
            ).into())
    }

    /// Find month by English name ("January", "February", etc.)
    pub fn from_english_name(name: &str) -> Result<Month> {
        Self::all_months()
            .iter()
            .find(|month| month.name_en.eq_ignore_ascii_case(name))
            .copied()
            .ok_or_else(|| UtilsError::Month(
                MonthError::not_valid_month_english(name.to_string())
            ).into())
    }

    /// Find month by Portuguese name ("Janeiro", "Fevereiro", etc.)
    pub fn from_portuguese_name(name: &str) -> Result<Month> {
        Self::all_months()
            .iter()
            .find(|month| month.name_ptbr.to_lowercase() == name.to_lowercase())
            .copied()
            .ok_or_else(|| UtilsError::Month(
                MonthError::not_valid_month_portuguese(name.to_string())
            ).into())
    }

    /// Find month by abbreviation ("Jan", "Feb", etc.)
    pub fn from_abbreviation(abbr: &str) -> Result<Month> {
        Self::all_months()
            .iter()
            .find(|month| month.name_short.eq_ignore_ascii_case(abbr))
            .copied()
            .ok_or_else(|| UtilsError::Month(
                MonthError::not_valid_month_abbreviation(abbr.to_string())
            ).into())
    }
    
    // Private methods made public for trait implementations
    pub fn is_valid_month_number(month: u8) -> bool {
        month >= 1 && month <= 12
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
        if let Ok(num) = input.parse::<u8>() {
            Self::is_valid_month_number(num)
        } else {
            false
        }
    }

    /// Public generic validation method - accepts both u8 and string types
    pub fn is_valid<T: MonthValidatable>(input: T) -> bool {
        input.is_valid_month()
    }
}