#[cfg(test)]
mod tests {
    use crate::utils::time::{Day, Month, Year};
    use chrono::{NaiveDate, Datelike, Timelike};

    #[test]
    fn test_all_days() {
        let days = Day::all_days();
        assert_eq!(days.len(), 31);
        assert_eq!(days[0].day, 1);
        assert_eq!(days[0].text, "01");
        assert_eq!(days[30].day, 31);
        assert_eq!(days[30].text, "31");
    }

    #[test]
    fn test_from_number() {
        let day1 = Day::from_number(1).unwrap();
        assert_eq!(day1.day, 1);
        assert_eq!(day1.text, "01");
        assert_eq!(day1.ordinal_en, "1st");
        assert_eq!(day1.ordinal_ptbr, "1º");

        let day31 = Day::from_number(31).unwrap();
        assert_eq!(day31.day, 31);
        assert_eq!(day31.text, "31");
        assert_eq!(day31.ordinal_en, "31st");
        assert_eq!(day31.ordinal_ptbr, "31º");

        // Invalid cases
        assert!(Day::from_number(0).is_err());
        assert!(Day::from_number(32).is_err());
    }

    #[test]
    fn test_ordinal_formatting() {
        // Test English ordinals
        assert_eq!(Day::from_number(1).unwrap().ordinal_en, "1st");
        assert_eq!(Day::from_number(2).unwrap().ordinal_en, "2nd");
        assert_eq!(Day::from_number(3).unwrap().ordinal_en, "3rd");
        assert_eq!(Day::from_number(4).unwrap().ordinal_en, "4th");
        assert_eq!(Day::from_number(21).unwrap().ordinal_en, "21st");
        assert_eq!(Day::from_number(22).unwrap().ordinal_en, "22nd");
        assert_eq!(Day::from_number(23).unwrap().ordinal_en, "23rd");
        assert_eq!(Day::from_number(31).unwrap().ordinal_en, "31st");

        // Test Portuguese ordinals
        assert_eq!(Day::from_number(1).unwrap().ordinal_ptbr, "1º");
        assert_eq!(Day::from_number(15).unwrap().ordinal_ptbr, "15º");
        assert_eq!(Day::from_number(31).unwrap().ordinal_ptbr, "31º");
    }

    #[test]
    fn test_from_generic() {
        // Parse from number
        let day = Day::from(15u8).unwrap();
        assert_eq!(day.day, 15);

        // Parse from string
        let day = Day::from("15").unwrap();
        assert_eq!(day.day, 15);

        let day = Day::from("01").unwrap();
        assert_eq!(day.day, 1);

        // Invalid cases
        assert!(Day::from("invalid").is_err());
        assert!(Day::from("32").is_err());
        assert!(Day::from(0u8).is_err());
    }

    #[test]
    fn test_is_valid_for_month() {
        let year_2024 = Year::from_number(2024).unwrap(); // Leap year
        let year_2023 = Year::from_number(2023).unwrap(); // Non-leap year
        let february = Month::from_number(2).unwrap();
        let april = Month::from_number(4).unwrap();
        let january = Month::from_number(1).unwrap();

        let day_28 = Day::from_number(28).unwrap();
        let day_29 = Day::from_number(29).unwrap();
        let day_30 = Day::from_number(30).unwrap();
        let day_31 = Day::from_number(31).unwrap();

        // February tests
        assert!(day_28.is_valid_for_month(&february, &year_2024));
        assert!(day_28.is_valid_for_month(&february, &year_2023));
        assert!(day_29.is_valid_for_month(&february, &year_2024)); // Valid in leap year
        assert!(!day_29.is_valid_for_month(&february, &year_2023)); // Invalid in non-leap year
        assert!(!day_30.is_valid_for_month(&february, &year_2024));

        // April tests (30 days)
        assert!(day_30.is_valid_for_month(&april, &year_2024));
        assert!(!day_31.is_valid_for_month(&april, &year_2024));

        // January tests (31 days)
        assert!(day_31.is_valid_for_month(&january, &year_2024));
    }

    #[test]
    fn test_is_valid_for_month_simple() {
        let february = Month::from_number(2).unwrap();
        let april = Month::from_number(4).unwrap();
        let january = Month::from_number(1).unwrap();

        let day_28 = Day::from_number(28).unwrap();
        let day_29 = Day::from_number(29).unwrap();
        let day_30 = Day::from_number(30).unwrap();
        let day_31 = Day::from_number(31).unwrap();

        // February (28 days in non-leap year logic)
        assert!(day_28.is_valid_for_month_simple(&february));
        assert!(!day_29.is_valid_for_month_simple(&february)); // Always invalid in simple logic

        // April (30 days)
        assert!(day_30.is_valid_for_month_simple(&april));
        assert!(!day_31.is_valid_for_month_simple(&april));

        // January (31 days)
        assert!(day_31.is_valid_for_month_simple(&january));
    }

    #[test]
    fn test_to_naive_date() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(6).unwrap();
        let day = Day::from_number(15).unwrap();

        let date = day.to_naive_date(&month, &year).unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2024, 6, 15).unwrap());

        // Invalid date
        let february = Month::from_number(2).unwrap();
        let day_30 = Day::from_number(30).unwrap();
        assert!(day_30.to_naive_date(&february, &year).is_err());
    }

    #[test]
    fn test_from_naive_date() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let day = Day::from_naive_date(&date).unwrap();
        assert_eq!(day.day, 15);
    }

    #[test]
    fn test_from_datetime_utc() {
        let datetime = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()
            .and_hms_opt(14, 30, 0).unwrap()
            .and_utc();
        let day = Day::from_datetime_utc(&datetime).unwrap();
        assert_eq!(day.day, 15);
    }

    #[test]
    fn test_from_datetime() {
        let datetime = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()
            .and_hms_opt(14, 30, 0).unwrap()
            .and_utc();
        let day = Day::from_datetime(&datetime).unwrap();
        assert_eq!(day.day, 15);
    }

    #[test]
    fn test_to_datetime_utc() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(6).unwrap();
        let day = Day::from_number(15).unwrap();

        let datetime = day.to_datetime_utc(&month, &year, 14, 30, 0).unwrap();
        let expected = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()
            .and_hms_opt(14, 30, 0).unwrap()
            .and_utc();
        assert_eq!(datetime, expected);
    }

    #[test]
    fn test_to_datetime_start_end_of_day() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(6).unwrap();
        let day = Day::from_number(15).unwrap();

        let start_of_day = day.to_datetime_start_of_day_utc(&month, &year).unwrap();
        let expected_start = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()
            .and_hms_opt(0, 0, 0).unwrap()
            .and_utc();
        assert_eq!(start_of_day, expected_start);

        let end_of_day = day.to_datetime_end_of_day_utc(&month, &year).unwrap();
        let expected_end = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()
            .and_hms_opt(23, 59, 59).unwrap()
            .and_utc();
        assert_eq!(end_of_day, expected_end);
    }

    #[test]
    fn test_to_weekday() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(6).unwrap(); // June
        let day = Day::from_number(15).unwrap(); // June 15, 2024 is a Saturday

        let weekday = day.to_weekday(&month, &year).unwrap();
        assert_eq!(weekday, chrono::Weekday::Sat);
    }

    #[test]
    fn test_weekday_names() {
        let year = Year::from_number(2024).unwrap();
        let month = Month::from_number(6).unwrap(); // June
        let day = Day::from_number(15).unwrap(); // June 15, 2024 is a Saturday

        // English names
        assert_eq!(day.to_weekday_name_en(&month, &year).unwrap(), "Saturday");
        assert_eq!(day.to_weekday_short_en(&month, &year).unwrap(), "Sat");

        // Portuguese names
        assert_eq!(day.to_weekday_name_ptbr(&month, &year).unwrap(), "Sábado");
        assert_eq!(day.to_weekday_short_ptbr(&month, &year).unwrap(), "Sáb");
    }

    #[test]
    fn test_navigation() {
        let day_15 = Day::from_number(15).unwrap();
        let day_1 = Day::from_number(1).unwrap();
        let day_31 = Day::from_number(31).unwrap();

        // Next day
        let next = day_15.next().unwrap();
        assert_eq!(next.day, 16);

        // Previous day
        let prev = day_15.previous().unwrap();
        assert_eq!(prev.day, 14);

        // Edge cases
        assert!(day_31.next().is_none()); // No day after 31
        assert!(day_1.previous().is_none()); // No day before 1
    }

    #[test]
    fn test_navigation_in_month() {
        let year = Year::from_number(2024).unwrap(); // Leap year
        let february = Month::from_number(2).unwrap();
        let april = Month::from_number(4).unwrap();

        let day_28 = Day::from_number(28).unwrap();
        let day_29 = Day::from_number(29).unwrap();
        let day_30 = Day::from_number(30).unwrap();

        // February in leap year
        let next_after_28 = day_28.next_in_month(&february, &year);
        assert_eq!(next_after_28.unwrap().day, 29); // Valid in leap year

        let next_after_29 = day_29.next_in_month(&february, &year);
        assert!(next_after_29.is_none()); // No February 30th

        // April (30 days)
        let next_after_30 = day_30.next_in_month(&april, &year);
        assert!(next_after_30.is_none()); // No April 31st

        // Previous is always valid if it exists
        let prev_29 = day_29.previous_in_month(&february, &year);
        assert_eq!(prev_29.unwrap().day, 28);
    }

    #[test]
    fn test_conversion_methods() {
        let day = Day::from_number(5).unwrap();

        assert_eq!(day.to_number(), 5);
        assert_eq!(day.to_text(), "05");
        assert_eq!(day.to_zero_padded_string(), "05");
        assert_eq!(day.to_number_string(), "5");
        assert_eq!(day.to_ordinal_en(), "5th");
        assert_eq!(day.to_ordinal_ptbr(), "5º");
    }

    #[test]
    fn test_is_valid() {
        // Works with u8
        assert!(Day::is_valid(15u8));
        assert!(!Day::is_valid(32u8));
        assert!(!Day::is_valid(0u8));

        // Works with &str
        assert!(Day::is_valid("15"));
        assert!(Day::is_valid("01"));
        assert!(Day::is_valid("31"));

        // Works with String
        assert!(Day::is_valid(String::from("15")));

        // Invalid cases
        assert!(!Day::is_valid("invalid"));
        assert!(!Day::is_valid("32"));
        assert!(!Day::is_valid("0"));
        assert!(!Day::is_valid(""));
    }

    #[test]
    fn test_day_validatable_trait() {
        use crate::utils::time::day::DayValidatable;

        // u8 validation
        assert!(15u8.is_valid_day());
        assert!(!32u8.is_valid_day());
        assert!(!0u8.is_valid_day());

        // str validation
        assert!("15".is_valid_day());
        assert!("01".is_valid_day());
        assert!(!"invalid".is_valid_day());
        assert!(!"32".is_valid_day());

        // String validation
        assert!(String::from("15").is_valid_day());
    }

    #[test]
    fn test_day_from_input_trait() {
        use crate::utils::time::day::DayFromInput;

        // u8 parsing
        assert!(15u8.parse_day().is_ok());
        assert!(32u8.parse_day().is_err());
        assert!(0u8.parse_day().is_err());

        // str parsing
        assert!("15".parse_day().is_ok());
        assert!("01".parse_day().is_ok());
        assert!("invalid".parse_day().is_err());
        assert!("32".parse_day().is_err());

        // String parsing
        assert!(String::from("15").parse_day().is_ok());
    }

    #[test]
    fn test_leap_year_edge_cases() {
        let leap_year = Year::from_number(2024).unwrap();
        let non_leap_year = Year::from_number(2023).unwrap();
        let february = Month::from_number(2).unwrap();
        let day_29 = Day::from_number(29).unwrap();

        // February 29th should be valid in leap year
        assert!(day_29.is_valid_for_month(&february, &leap_year));
        let date_leap = day_29.to_naive_date(&february, &leap_year).unwrap();
        assert_eq!(date_leap, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());

        // February 29th should be invalid in non-leap year
        assert!(!day_29.is_valid_for_month(&february, &non_leap_year));
        assert!(day_29.to_naive_date(&february, &non_leap_year).is_err());
    }

    #[test]
    fn test_month_boundary_cases() {
        let year = Year::from_number(2024).unwrap();
        let day_31 = Day::from_number(31).unwrap();

        // Months with 31 days
        for month_num in [1, 3, 5, 7, 8, 10, 12] {
            let month = Month::from_number(month_num).unwrap();
            assert!(day_31.is_valid_for_month(&month, &year));
        }

        // Months with 30 days
        for month_num in [4, 6, 9, 11] {
            let month = Month::from_number(month_num).unwrap();
            assert!(!day_31.is_valid_for_month(&month, &year));
        }

        // February (28/29 days)
        let february = Month::from_number(2).unwrap();
        assert!(!day_31.is_valid_for_month(&february, &year));
    }

    #[test]
    fn test_comprehensive_date_creation() {
        // Test creating various dates and ensuring they work correctly
        let test_cases = vec![
            (2024, 1, 1),   // New Year's Day
            (2024, 2, 29),  // Leap day
            (2024, 12, 31), // New Year's Eve
            (2024, 6, 15),  // Mid-year date
        ];

        for (year_num, month_num, day_num) in test_cases {
            let year = Year::from_number(year_num).unwrap();
            let month = Month::from_number(month_num).unwrap();
            let day = Day::from_number(day_num).unwrap();

            // Validate the combination
            assert!(day.is_valid_for_month(&month, &year));

            // Create NaiveDate
            let date = day.to_naive_date(&month, &year).unwrap();
            assert_eq!(date.year(), year_num);
            assert_eq!(date.month(), month_num as u32);
            assert_eq!(date.day(), day_num as u32);

            // Create DateTime
            let datetime = day.to_datetime_utc(&month, &year, 12, 0, 0).unwrap();
            assert_eq!(datetime.year(), year_num);
            assert_eq!(datetime.month(), month_num as u32);
            assert_eq!(datetime.day(), day_num as u32);
            assert_eq!(datetime.hour(), 12);

            // Round trip test
            let extracted_day = Day::from_naive_date(&date).unwrap();
            assert_eq!(extracted_day.day, day_num);
        }
    }
}