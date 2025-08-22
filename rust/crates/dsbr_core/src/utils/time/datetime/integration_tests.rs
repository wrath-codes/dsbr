#[cfg(test)]
mod integration_tests {
    use crate::utils::time::{DateTime, DateTimeBuilder, Duration, Year, Month, Day};

    #[test]
    fn test_duration_extraction() {
        let dt = DateTime::from("2024-03-15T14:30:45Z").unwrap();

        // Basic extraction
        let time_of_day = dt.extract_time();
        assert_eq!(time_of_day.total_hours(), 14);
        assert_eq!(time_of_day.minutes(), 30);
        assert_eq!(time_of_day.seconds(), 45);

        let since_midnight = dt.time_since_midnight();
        assert_eq!(since_midnight.total_hours(), 14);
        assert_eq!(since_midnight.minutes(), 30);

        let until_midnight = dt.time_until_midnight();
        assert_eq!(until_midnight.total_hours(), 9); // 24 - 14 - 1 (for the 30m45s)
    }

    #[test]
    fn test_period_based_extraction() {
        let dt = DateTime::from("2024-03-15T14:30:45Z").unwrap();

        // Since start of month (March 15th = 14 days + time)
        let since_month_start = dt.time_since_month_start().unwrap();
        assert_eq!(since_month_start.total_days(), 14); // 15th - 1st = 14 days

        // Since start of year
        let since_year_start = dt.time_since_year_start().unwrap();
        let expected_days = 31 + 29 + 14; // Jan (31) + Feb (29, leap year) + 14 days in March
        assert_eq!(since_year_start.total_days(), expected_days);
    }

    #[test]
    fn test_time_until_periods() {
        let dt = DateTime::from("2024-03-15T14:30:45Z").unwrap();

        // Until end of day
        let until_end_of_day = dt.time_until_end_of_day();
        assert_eq!(until_end_of_day.total_hours(), 9); // Approximately 9 hours left

        // Until end of month (March has 31 days)
        let until_month_end = dt.time_until_month_end().unwrap();
        let expected_days = 31 - 15; // Days remaining in March
        assert_eq!(until_month_end.total_days(), expected_days);
    }

    #[test]
    fn test_datetime_builder_basic() {
        let dt = DateTime::builder()
            .date(2024, 3, 15).unwrap()
            .time(14, 30, 45)
            .build().unwrap();

        assert_eq!(dt.year().year, 2024);
        assert_eq!(dt.month().month, 3);
        assert_eq!(dt.day().day, 15);
        assert_eq!(dt.hour(), 14);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
    }

    #[test]
    fn test_datetime_builder_with_duration() {
        let morning_time = Duration::from_components(9, 30, 0, 0, 0);
        let dt = DateTime::builder()
            .date(2024, 3, 15).unwrap()
            .at_time(morning_time)
            .build().unwrap();

        assert_eq!(dt.hour(), 9);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 0);
    }

    #[test]
    fn test_datetime_builder_convenience_methods() {
        let dt_noon = DateTime::builder()
            .date(2024, 3, 15).unwrap()
            .at_noon()
            .build().unwrap();

        assert_eq!(dt_noon.hour(), 12);
        assert_eq!(dt_noon.minute(), 0);

        let dt_midnight = DateTime::builder()
            .date(2024, 3, 15).unwrap()
            .at_midnight()
            .build().unwrap();

        assert_eq!(dt_midnight.hour(), 0);
        assert_eq!(dt_midnight.minute(), 0);
    }

    #[test]
    fn test_datetime_builder_with_existing_types() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(3).unwrap();
        let day = Day::from_number(15).unwrap();

        let dt = DateTime::builder()
            .year(year)
            .month(month)
            .day(day)
            .time(16, 45, 0)
            .build().unwrap();

        assert_eq!(dt.year().year, 2024);
        assert_eq!(dt.month().month, 3);
        assert_eq!(dt.day().day, 15);
        assert_eq!(dt.hour(), 16);
        assert_eq!(dt.minute(), 45);
    }

    #[test]
    fn test_extract_and_rebuild_pattern() {
        let source_dt = DateTime::from("2024-03-15T14:30:45Z").unwrap();
        let target_date = DateTime::from("2024-03-20").unwrap();

        // Extract time from source and apply to target date
        let extracted_time = source_dt.extract_time();
        let new_dt = DateTime::builder()
            .year(target_date.year)
            .month(target_date.month)
            .day(target_date.day)
            .at_time(extracted_time)
            .build().unwrap();

        assert_eq!(new_dt.year().year, 2024);
        assert_eq!(new_dt.month().month, 3);
        assert_eq!(new_dt.day().day, 20);
        assert_eq!(new_dt.hour(), 14);
        assert_eq!(new_dt.minute(), 30);
        assert_eq!(new_dt.second(), 45);
    }

    #[test]
    fn test_time_calculations() {
        let dt = DateTime::from("2024-03-15T14:30:45Z").unwrap();

        // Time left in day
        let remaining_today = dt.time_until_end_of_day();
        assert!(remaining_today.total_hours() < 10); // Less than 10 hours left

        // Time since start of week (Friday = 4 days since Monday)
        let since_week_start = dt.time_since_week_start().unwrap();
        // March 15, 2024 is a Friday, so 4 days since Monday
        assert_eq!(since_week_start.total_days(), 4);
    }

    #[test]
    fn test_builder_error_handling() {
        // Missing required fields should error
        let result = DateTime::builder()
            .hour(14)
            .minute(30)
            .build();
        
        assert!(result.is_err()); // Should fail because year, month, day are missing
    }

    #[test]
    fn test_builder_today_tomorrow() {
        // These tests depend on current time, so we'll just verify they don't panic
        let today_builder = DateTimeBuilder::today();
        assert!(today_builder.is_ok());

        let tomorrow_builder = DateTimeBuilder::tomorrow();
        assert!(tomorrow_builder.is_ok());

        if let (Ok(today), Ok(tomorrow)) = (today_builder, tomorrow_builder) {
            let today_dt = today.at_noon().build().unwrap();
            let tomorrow_dt = tomorrow.at_noon().build().unwrap();

            // Tomorrow should be one day after today
            let diff = tomorrow_dt.duration_since(&today_dt);
            assert!(diff.is_some());
            if let Some(duration) = diff {
                assert_eq!(duration.total_days(), 1);
            }
        }
    }

    #[test]
    fn test_combined_usage_patterns() {
        // Create a meeting for same time tomorrow
        let now = DateTime::from("2024-03-15T14:30:45Z").unwrap();
        let extracted_time = now.extract_time();

        let tomorrow = now.add_days(1).unwrap();
        let same_time_tomorrow = DateTime::builder()
            .year(tomorrow.0)
            .month(tomorrow.1)
            .day(tomorrow.2)
            .at_time(extracted_time)
            .build().unwrap();

        assert_eq!(same_time_tomorrow.day().day, 16); // Next day
        assert_eq!(same_time_tomorrow.hour(), 14); // Same time
        assert_eq!(same_time_tomorrow.minute(), 30);
        assert_eq!(same_time_tomorrow.second(), 45);
    }

    #[test]
    fn test_duration_formatting_with_extracted_time() {
        let dt = DateTime::from("2024-03-15T14:30:45Z").unwrap();
        let time_of_day = dt.extract_time();

        assert_eq!(time_of_day.to_hms(), "14:30:45");
        assert_eq!(time_of_day.to_readable(), "14h 30m 45s");
    }
}