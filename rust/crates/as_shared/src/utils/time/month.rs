use dashmap::{DashSet};
use std::sync::LazyLock;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::core::{Result};
use crate::utils::{UtilsError};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct Month {
    pub month: u8,
    pub text: &'static str,
    pub name_ptbr: &'static str,
    pub name_short: &'static str,
    pub name_en: &'static str,
}


#[derive(Error, Debug)]
pub enum MonthError {
    #[error("No such month: {0}")]
    NoSuchMonth(String),
    
    #[error("Not a valid month: {0}. This value cannot be converted to a month.")]
    NotValidMonth(String),

    #[error("Not a valid month: {0}. Must be a number between 1 and 12.")]
    NotValidMonthNumber(String),

    #[error("Not a valid month: {0}. Must be a string between '01' and '12'.")]
    NotValidMonthString(String),

    #[error("Not a valid month: {0}. Must be an abbreviation between 'Jan' and 'Dec'.")]
    NotValidMonthAbbreviation(String),

    #[error("Not a valid month: {0}. Must be a month in english between 'January' and 'December'.")]
    NotValidMonthEnglish(String),

    #[error("Not a valid month: {0}. Must be a month in portuguese between 'Janeiro' and 'Dezembro'.")]
    NotValidMonthPortuguese(String),
    
    #[error("Cannot parse month: {0}")]
    CannotParseMonth(String),
    
    #[error("Cannot convert month: {0}")]
    CannotConvertMonth(String),
}

/// Trait for types that can be validated as months
/// 
/// # Examples
/// 
/// ```
/// use crate::arrow-sus-shared::utils::{Month, MonthValidatable};
/// 
/// // u8 validation
/// assert!(5u8.is_valid_month());
/// assert!(!13u8.is_valid_month());
/// 
/// // str validation
/// assert!("January".is_valid_month());
/// assert!("01".is_valid_month());
/// assert!(!"invalid".is_valid_month());
/// 
/// // String validation
/// assert!(String::from("February").is_valid_month());
/// ```
pub trait MonthValidatable {
    fn is_valid_month(&self) -> bool;
}

impl MonthValidatable for u8 {
    fn is_valid_month(&self) -> bool {
        *self >= 1 && *self <= 12
    }
}

impl MonthValidatable for str {
    fn is_valid_month(&self) -> bool {
        // Try all validation methods
        Month::is_valid_month_text(self) ||
        Month::is_valid_english_name(self) ||
        Month::is_valid_portuguese_name(self) ||
        Month::is_valid_abbreviation(self) ||
        Month::is_valid_month_number_string(self)
    }
}

impl MonthValidatable for String {
    fn is_valid_month(&self) -> bool {
        self.as_str().is_valid_month()
    }
}



impl MonthError {
    pub fn no_such_month<S: Into<String>>(msg: S) -> Self { Self::NoSuchMonth(msg.into()) }
    pub fn not_valid_month<S: Into<String>>(msg: S) -> Self { Self::NotValidMonth(msg.into()) }
    pub fn not_valid_month_number<S: Into<String>>(msg: S) -> Self { Self::NotValidMonthNumber(msg.into()) }
    pub fn not_valid_month_string<S: Into<String>>(msg: S) -> Self { Self::NotValidMonthString(msg.into()) }
    pub fn not_valid_month_abbreviation<S: Into<String>>(msg: S) -> Self { Self::NotValidMonthAbbreviation(msg.into()) }
    pub fn not_valid_month_english<S: Into<String>>(msg: S) -> Self { Self::NotValidMonthEnglish(msg.into()) }
    pub fn not_valid_month_portuguese<S: Into<String>>(msg: S) -> Self { Self::NotValidMonthPortuguese(msg.into()) }
    pub fn cannot_parse_month<S: Into<String>>(msg: S) -> Self { Self::CannotParseMonth(msg.into()) }
    pub fn cannot_convert_month<S: Into<String>>(msg: S) -> Self { Self::CannotConvertMonth(msg.into()) }
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
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// let months = Month::all_months();
    /// assert_eq!(months.len(), 12);
    /// assert_eq!(months[0].month, 1);
    /// assert_eq!(months[0].name_en, "January");
    /// assert_eq!(months[11].month, 12);
    /// assert_eq!(months[11].name_en, "December");
    /// ```
    pub fn all_months() -> &'static [Month; 12] {
        &MONTHS_ORDERED
    }
   
    /// Get the next month (December wraps to January)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::time::Month;
    /// 
    /// let january = Month::from_number(1).unwrap();
    /// let february = january.next();
    /// assert_eq!(february.month, 2);
    /// assert_eq!(february.name_en, "February");
    /// 
    /// // Test wrapping
    /// let december = Month::from_number(12).unwrap();
    /// let january_again = december.next();
    /// assert_eq!(january_again.month, 1);
    /// assert_eq!(january_again.name_en, "January");
    /// ```
    pub fn next(&self) -> Month {
        let months = Self::all_months();
        let current_index = (self.month - 1) as usize;
        let next_index = (current_index + 1) % 12;
        months[next_index]
    }

    /// Get the previous month (January wraps to December)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// let february = Month::from_number(2).unwrap();
    /// let january = february.previous();
    /// assert_eq!(january.month, 1);
    /// assert_eq!(january.name_en, "January");
    /// 
    /// // Test wrapping
    /// let january = Month::from_number(1).unwrap();
    /// let december = january.previous();
    /// assert_eq!(december.month, 12);
    /// assert_eq!(december.name_en, "December");
    /// ```
    pub fn previous(&self) -> Month {
        let months = Self::all_months();
        let current_index = (self.month - 1) as usize;
        let prev_index = if current_index == 0 { 11 } else { current_index - 1 };
        months[prev_index]
    }
    
    /// Check if this month comes before another chronologically
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::time::Month;
    /// 
    /// let january = Month::from_number(1).unwrap();
    /// let march = Month::from_number(3).unwrap();
    /// 
    /// assert!(january.is_before(&march));
    /// assert!(!march.is_before(&january));
    /// assert!(!january.is_before(&january));
    /// ```
    pub fn is_before(&self, other: &Month) -> bool {
        self.month < other.month
    }

    /// Check if this month comes after another chronologically
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// let january = Month::from_number(1).unwrap();
    /// let march = Month::from_number(3).unwrap();
    /// 
    /// assert!(march.is_after(&january));
    /// assert!(!january.is_after(&march));
    /// assert!(!january.is_after(&january));
    /// ```
    pub fn is_after(&self, other: &Month) -> bool {
        self.month > other.month
    }

    /// Get the number of months between this month and another 
    /// (positive = forward, negative = backward)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// let january = Month::from_number(1).unwrap();
    /// let march = Month::from_number(3).unwrap();
    /// let november = Month::from_number(11).unwrap();
    /// 
    /// assert_eq!(january.months_until(&march), 2);
    /// assert_eq!(march.months_until(&january), -2);
    /// assert_eq!(january.months_until(&november), 10);
    /// assert_eq!(january.months_until(&january), 0);
    /// ```
    pub fn months_until(&self, other: &Month) -> i8 {
        other.month as i8 - self.month as i8
    }

    /// Get the number of months since another month
    /// (positive = this month is later, negative = this month is earlier)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// let january = Month::from_number(1).unwrap();
    /// let march = Month::from_number(3).unwrap();
    /// 
    /// assert_eq!(march.months_since(&january), 2);
    /// assert_eq!(january.months_since(&march), -2);
    /// assert_eq!(january.months_since(&january), 0);
    /// ```
    pub fn months_since(&self, other: &Month) -> i8 {
        self.month as i8 - other.month as i8
    }

    /// Convert to zero-padded string format ("01", "02", etc.)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// let january = Month::from_number(1).unwrap();
    /// let december = Month::from_number(12).unwrap();
    /// 
    /// assert_eq!(january.to_zero_padded_string(), "01");
    /// assert_eq!(december.to_zero_padded_string(), "12");
    /// ```
    pub fn to_zero_padded_string(&self) -> String {
        self.text.to_string()
    }

    /// Convert to number string format ("1", "2", etc.)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// let january = Month::from_number(1).unwrap();
    /// let december = Month::from_number(12).unwrap();
    /// 
    /// assert_eq!(january.to_number_string(), "1");
    /// assert_eq!(december.to_number_string(), "12");
    /// ```
    pub fn to_number_string(&self) -> String {
        self.month.to_string()
    }
    
    /// Find month by number (1-12)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::time::Month;
    /// 
    /// let january = Month::from_number(1).unwrap();
    /// assert_eq!(january.month, 1);
    /// assert_eq!(january.name_en, "January");
    /// 
    /// let december = Month::from_number(12).unwrap();
    /// assert_eq!(december.month, 12);
    /// assert_eq!(december.name_en, "December");
    /// 
    /// // Invalid cases
    /// assert!(Month::from_number(0).is_err());
    /// assert!(Month::from_number(13).is_err());
    /// ```
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
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// let january = Month::from_text("01").unwrap();
    /// assert_eq!(january.month, 1);
    /// assert_eq!(january.name_en, "January");
    /// 
    /// let december = Month::from_text("12").unwrap();
    /// assert_eq!(december.month, 12);
    /// 
    /// // Invalid cases
    /// assert!(Month::from_text("00").is_err());
    /// assert!(Month::from_text("13").is_err());
    /// assert!(Month::from_text("1").is_err()); // Must be zero-padded
    /// ```
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
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// let january = Month::from_english_name("January").unwrap();
    /// assert_eq!(january.month, 1);
    /// 
    /// // Case insensitive
    /// let february = Month::from_english_name("february").unwrap();
    /// assert_eq!(february.month, 2);
    /// 
    /// let march = Month::from_english_name("MARCH").unwrap();
    /// assert_eq!(march.month, 3);
    /// 
    /// // Invalid cases
    /// assert!(Month::from_english_name("Invalid").is_err());
    /// assert!(Month::from_english_name("Janeiro").is_err()); // Portuguese name
    /// ```
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
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// let january = Month::from_portuguese_name("Janeiro").unwrap();
    /// assert_eq!(january.month, 1);
    /// 
    /// // Case insensitive
    /// let february = Month::from_portuguese_name("fevereiro").unwrap();
    /// assert_eq!(february.month, 2);
    /// 
    /// let march = Month::from_portuguese_name("MARÇO").unwrap();
    /// assert_eq!(march.month, 3);
    /// 
    /// // Invalid cases
    /// assert!(Month::from_portuguese_name("Invalid").is_err());
    /// assert!(Month::from_portuguese_name("January").is_err()); // English name
    /// ```
    pub fn from_portuguese_name(name: &str) -> Result<Month> {
        Self::all_months()
            .iter()
            .find(|month| month.name_ptbr.eq_ignore_ascii_case(name))
            .copied()
            .ok_or_else(|| UtilsError::Month(
                MonthError::not_valid_month_portuguese(name.to_string())
            ).into())
    }

    /// Find month by abbreviation ("Jan", "Feb", etc.)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// let january = Month::from_abbreviation("Jan").unwrap();
    /// assert_eq!(january.month, 1);
    /// 
    /// // Case insensitive
    /// let february = Month::from_abbreviation("feb").unwrap();
    /// assert_eq!(february.month, 2);
    /// 
    /// let march = Month::from_abbreviation("MAR").unwrap();
    /// assert_eq!(march.month, 3);
    /// 
    /// // Invalid cases
    /// assert!(Month::from_abbreviation("Invalid").is_err());
    /// assert!(Month::from_abbreviation("January").is_err()); // Full name, not abbreviation
    /// ```
    pub fn from_abbreviation(abbr: &str) -> Result<Month> {
        Self::all_months()
            .iter()
            .find(|month| month.name_short.eq_ignore_ascii_case(abbr))
            .copied()
            .ok_or_else(|| UtilsError::Month(
                MonthError::not_valid_month_abbreviation(abbr.to_string())
            ).into())
    }
    
    // Private methods
    fn is_valid_month_number(month: u8) -> bool {
        month >= 1 && month <= 12
    }

    fn is_valid_month_text(text: &str) -> bool {
        Self::all_months()
            .iter()
            .any(|month| month.text == text)
    }

    fn is_valid_english_name(name: &str) -> bool {
        Self::all_months()
            .iter()
            .any(|month| month.name_en.eq_ignore_ascii_case(name))
    }

    fn is_valid_portuguese_name(name: &str) -> bool {
        Self::all_months()
            .iter()
            .any(|month| month.name_ptbr.eq_ignore_ascii_case(name))
    }

    fn is_valid_abbreviation(abbr: &str) -> bool {
        Self::all_months()
            .iter()
            .any(|month| month.name_short.eq_ignore_ascii_case(abbr))
    }

    fn is_valid_month_number_string(input: &str) -> bool {
        if let Ok(num) = input.parse::<u8>() {
            Self::is_valid_month_number(num)
        } else {
            false
        }
    }

    /// Public generic validation method - accepts both u8 and string types
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::arrow-sus-shared::utils::Month;
    /// 
    /// // Works with u8
    /// assert!(Month::is_valid(5u8));
    /// assert!(!Month::is_valid(13u8));
    /// 
    /// // Works with &str - various formats
    /// assert!(Month::is_valid("January"));
    /// assert!(Month::is_valid("01"));
    /// assert!(Month::is_valid("Jan"));
    /// assert!(Month::is_valid("Janeiro"));
    /// assert!(Month::is_valid("5"));
    /// 
    /// // Case insensitive
    /// assert!(Month::is_valid("january"));
    /// assert!(Month::is_valid("FEBRUARY"));
    /// 
    /// // Works with String
    /// assert!(Month::is_valid(String::from("March")));
    /// 
    /// // Invalid cases
    /// assert!(!Month::is_valid("invalid"));
    /// assert!(!Month::is_valid("13"));
    /// assert!(!Month::is_valid("0"));
    /// ```
    pub fn is_valid<T: MonthValidatable>(input: T) -> bool {
        input.is_valid_month()
    }
}