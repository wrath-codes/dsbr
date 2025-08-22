use thiserror::Error;

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