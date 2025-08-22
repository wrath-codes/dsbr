#[cfg(test)]
mod tests {
    use crate::utils::time::{DateTime, DateTimeFormat, Duration, Year, Month, Day};

    #[test]
    fn test_datetime_creation() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(3).unwrap();
        let day = Day::from_number(15).unwrap();
        
        let dt = DateTime::new(year, month, day, 14, 30, 45, 123_456_789).unwrap();
        
        assert_eq!(dt.year().year, 2024);
        assert_eq!(dt.month().month, 3);
        assert_eq!(dt.day().day, 15);
        assert_eq!(dt.hour(), 14);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
        assert_eq!(dt.nanosecond(), 123_456_789);
    }

    #[test]
    fn test_datetime_validation() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(3).unwrap();
        let day = Day::from_number(15).unwrap();
        
        // Valid time components
        assert!(DateTime::new(year, month, day, 23, 59, 59, 999_999_999).is_ok());
        
        // Invalid hour
        assert!(DateTime::new(year, month, day, 24, 0, 0, 0).is_err());
        
        // Invalid minute
        assert!(DateTime::new(year, month, day, 0, 60, 0, 0).is_err());
        
        // Invalid second
        assert!(DateTime::new(year, month, day, 0, 0, 60, 0).is_err());
        
        // Invalid nanosecond
        assert!(DateTime::new(year, month, day, 0, 0, 0, 1_000_000_000).is_err());
    }

    #[test]
    fn test_datetime_from_timestamp() {
        let timestamp = 1710504645i64; // March 15, 2024 14:30:45 UTC
        let dt = DateTime::from_timestamp(timestamp).unwrap();
        
        assert_eq!(dt.year().year, 2024);
        assert_eq!(dt.month().month, 3);
        assert_eq!(dt.day().day, 15);
    }

    #[test]
    fn test_datetime_iso8601_parsing() {
        let dt1 = DateTime::from_iso8601("2024-03-15T14:30:45Z").unwrap();
        assert_eq!(dt1.year().year, 2024);
        assert_eq!(dt1.month().month, 3);
        assert_eq!(dt1.day().day, 15);
        assert_eq!(dt1.hour(), 14);
        assert_eq!(dt1.minute(), 30);
        assert_eq!(dt1.second(), 45);

        let dt2 = DateTime::from_iso8601("2024-03-15T14:30:45.123456789Z").unwrap();
        assert_eq!(dt2.nanosecond(), 123_456_789);

        let dt3 = DateTime::from_iso8601("2024-03-15").unwrap();
        assert_eq!(dt3.hour(), 0);
        assert_eq!(dt3.minute(), 0);
        assert_eq!(dt3.second(), 0);
    }

    #[test]
    fn test_datetime_format_parsing() {
        let dt1 = DateTime::from_format("20240315", DateTimeFormat::YYYYMMDD).unwrap();
        assert_eq!(dt1.year().year, 2024);
        assert_eq!(dt1.month().month, 3);
        assert_eq!(dt1.day().day, 15);

        let dt2 = DateTime::from_format("2024-03-15", DateTimeFormat::YYYY_MM_DD).unwrap();
        assert_eq!(dt2.year().year, 2024);
        assert_eq!(dt2.month().month, 3);
        assert_eq!(dt2.day().day, 15);

        let dt3 = DateTime::from_format("15/03/2024", DateTimeFormat::DD_MM_YYYY).unwrap();
        assert_eq!(dt3.year().year, 2024);
        assert_eq!(dt3.month().month, 3);
        assert_eq!(dt3.day().day, 15);

        let dt4 = DateTime::from_format("03/15/2024", DateTimeFormat::MM_DD_YYYY).unwrap();
        assert_eq!(dt4.year().year, 2024);
        assert_eq!(dt4.month().month, 3);
        assert_eq!(dt4.day().day, 15);
    }

    #[test]
    fn test_datetime_yymm_parsing() {
        let dt = DateTime::from_format("2403", DateTimeFormat::YYMM).unwrap();
        assert_eq!(dt.year().year, 2024);
        assert_eq!(dt.month().month, 3);
        assert_eq!(dt.day().day, 1); // First day of month
        assert_eq!(dt.hour(), 0); // Start of day
        assert_eq!(dt.minute(), 0);
        assert_eq!(dt.second(), 0);
    }

    #[test]
    fn test_datetime_formatting() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(3).unwrap();
        let day = Day::from_number(15).unwrap();
        let dt = DateTime::new(year, month, day, 14, 30, 45, 123_456_789).unwrap();

        assert_eq!(dt.to_format(DateTimeFormat::ISO8601).unwrap(), "2024-03-15T14:30:45.123456789Z");
        assert_eq!(dt.to_format(DateTimeFormat::YYYYMMDD).unwrap(), "20240315");
        assert_eq!(dt.to_format(DateTimeFormat::YYYY_MM_DD).unwrap(), "2024-03-15");
        assert_eq!(dt.to_format(DateTimeFormat::DD_MM_YYYY).unwrap(), "15/03/2024");
        assert_eq!(dt.to_format(DateTimeFormat::MM_DD_YYYY).unwrap(), "03/15/2024");
        assert_eq!(dt.to_format(DateTimeFormat::YYMM).unwrap(), "2403");
    }

    #[test]
    fn test_datetime_duration_arithmetic() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(3).unwrap();
        let day = Day::from_number(15).unwrap();
        let dt = DateTime::new(year, month, day, 14, 30, 0, 0).unwrap();

        let duration = Duration::from_hours(2);
        let later = dt.add_duration(&duration).unwrap();
        
        assert_eq!(later.hour(), 16);
        assert_eq!(later.minute(), 30);
        assert_eq!(later.second(), 0);

        let much_later = dt.add_hours(25).unwrap(); // Should roll over to next day
        assert_eq!(much_later.day().day, 16);
        assert_eq!(much_later.hour(), 15); // 14 + 25 - 24 = 15
    }

    #[test]
    fn test_datetime_time_operations() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(3).unwrap();
        let day = Day::from_number(15).unwrap();
        let dt = DateTime::new(year, month, day, 14, 30, 45, 0).unwrap();

        let time_since_midnight = dt.time_since_midnight();
        assert_eq!(time_since_midnight.total_hours(), 14);
        assert_eq!(time_since_midnight.minutes(), 30);
        assert_eq!(time_since_midnight.seconds(), 45);

        let time_until_midnight = dt.time_until_midnight();
        assert_eq!(time_until_midnight.total_hours(), 9); // 24 - 14 - 1 (for the 30m45s)
    }

    #[test]
    fn test_datetime_auto_detection() {
        let dt1 = DateTime::from("2024-03-15T14:30:45Z").unwrap();
        assert_eq!(dt1.year().year, 2024);

        let dt2 = DateTime::from("2024-03-15").unwrap();
        assert_eq!(dt2.year().year, 2024);

        let dt3 = DateTime::from("15/03/2024").unwrap();
        assert_eq!(dt3.year().year, 2024);

        let dt4 = DateTime::from("2403").unwrap(); // YYMM format
        assert_eq!(dt4.year().year, 2024);
        assert_eq!(dt4.month().month, 3);
    }

    #[test]
    fn test_datetime_readable_formatting() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(3).unwrap();
        let day = Day::from_number(15).unwrap();
        let dt = DateTime::new(year, month, day, 14, 30, 45, 0).unwrap();

        let readable_en = dt.to_readable_en();
        assert!(readable_en.contains("March"));
        assert!(readable_en.contains("15"));
        assert!(readable_en.contains("2024"));
        assert!(readable_en.contains("14:30:45"));

        let readable_ptbr = dt.to_readable_ptbr();
        assert!(readable_ptbr.contains("Março"));
        assert!(readable_ptbr.contains("15"));
        assert!(readable_ptbr.contains("2024"));
        assert!(readable_ptbr.contains("14:30:45"));
    }

    #[test]
    fn test_datetime_timestamp_conversion() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(3).unwrap();
        let day = Day::from_number(15).unwrap();
        let dt = DateTime::new(year, month, day, 14, 30, 45, 123_456_789).unwrap();

        let timestamp = dt.to_timestamp().unwrap();
        let (ts_secs, ts_nanos) = dt.to_timestamp_nanos().unwrap();
        
        assert_eq!(timestamp, ts_secs);
        assert_eq!(ts_nanos, 123_456_789);

        // Round trip test
        let dt_from_timestamp = DateTime::from_timestamp_nanos(ts_secs, ts_nanos).unwrap();
        assert_eq!(dt.year().year, dt_from_timestamp.year().year);
        assert_eq!(dt.month().month, dt_from_timestamp.month().month);
        assert_eq!(dt.day().day, dt_from_timestamp.day().day);
        assert_eq!(dt.hour(), dt_from_timestamp.hour());
        assert_eq!(dt.minute(), dt_from_timestamp.minute());
        assert_eq!(dt.second(), dt_from_timestamp.second());
        assert_eq!(dt.nanosecond(), dt_from_timestamp.nanosecond());
    }
}