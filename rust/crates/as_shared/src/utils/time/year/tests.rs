#[cfg(test)]
mod tests {
    use crate::utils::time::Year;
    use crate::utils::time::Month;
    use chrono::{NaiveDate, Utc, Local, Datelike};

    #[test]
    fn test_year_validatable_i32() {
        use crate::utils::time::year::YearValidatable;
        
        // i32 validation
        assert!(2023i32.is_valid_year());
        assert!(!1800i32.is_valid_year());
    }

    #[test]
    fn test_year_validatable_str() {
        use crate::utils::time::year::YearValidatable;
        
        // str validation
        assert!("2023".is_valid_year());
        assert!("23".is_valid_2digit_year());
        assert!(!"invalid".is_valid_year());
    }

    #[test]
    fn test_year_validatable_string() {
        use crate::utils::time::year::YearValidatable;
        
        // String validation
        assert!(String::from("2024").is_valid_year());
    }

    #[test]
    fn test_year_from_input_i32() {
        use crate::utils::time::year::YearFromInput;
        
        // i32 parsing
        assert!(2023i32.parse_year().is_ok());
        assert!(1800i32.parse_year().is_err());
    }

    #[test]
    fn test_year_from_input_str() {
        use crate::utils::time::year::YearFromInput;
        
        // str parsing
        assert!("2023".parse_year().is_ok());
        assert!("23".parse_year().is_ok()); // 2-digit year
        assert!("invalid".parse_year().is_err());
    }

    #[test]
    fn test_year_from_input_string() {
        use crate::utils::time::year::YearFromInput;
        
        // String parsing
        assert!(String::from("2024").parse_year().is_ok());
    }

    #[test]
    fn test_all_years() {
        let years = Year::all_years();
        assert_eq!(years.len(), 201);
        assert_eq!(years[0].year, 1900);
        assert_eq!(years[200].year, 2100);
    }

    #[test]
    fn test_from_generic() {
        // Parse from number
        let year = Year::from(2023i32).unwrap();
        assert_eq!(year.year, 2023);

        // Parse from 4-digit string
        let year = Year::from("2024").unwrap();
        assert_eq!(year.year, 2024);

        // Parse from 2-digit string (with pivot logic)
        let year = Year::from("23").unwrap();
        assert_eq!(year.year, 2023); // 00-49 = 2000-2049

        let year = Year::from("99").unwrap();
        assert_eq!(year.year, 1999); // 50-99 = 1950-1999

        // Invalid cases
        assert!(Year::from("invalid").is_err());
        assert!(Year::from("1800").is_err());
        assert!(Year::from(1800i32).is_err());
    }

    #[test]
    fn test_from_number() {
        let year = Year::from_number(2023).unwrap();
        assert_eq!(year.year, 2023);
        assert_eq!(year.text_4d, "2023");
        assert_eq!(year.text_2d, "23");

        // Invalid cases
        assert!(Year::from_number(1800).is_err());
        assert!(Year::from_number(2200).is_err());
    }

    #[test]
    fn test_from_2digit_number() {
        // 00-49 maps to 2000-2049
        let year = Year::from_2digit_number(23).unwrap();
        assert_eq!(year.year, 2023);

        // 50-99 maps to 1950-1999
        let year = Year::from_2digit_number(99).unwrap();
        assert_eq!(year.year, 1999);

        // Invalid cases
        assert!(Year::from_2digit_number(100).is_err());
        assert!(Year::from_2digit_number(-1).is_err());
    }

    #[test]
    fn test_next() {
        let year = Year::from_number(2023).unwrap();
        let next = year.next().unwrap();
        assert_eq!(next.year, 2024);

        // Test boundary
        let year = Year::from_number(2100).unwrap();
        assert!(year.next().is_err()); // Beyond max year
    }

    #[test]
    fn test_previous() {
        let year = Year::from_number(2023).unwrap();
        let prev = year.previous().unwrap();
        assert_eq!(prev.year, 2022);

        // Test boundary
        let year = Year::from_number(1900).unwrap();
        assert!(year.previous().is_err()); // Below min year
    }

    #[test]
    fn test_add_years() {
        let year = Year::from_number(2020).unwrap();
        let future = year.add_years(5).unwrap();
        assert_eq!(future.year, 2025);

        // Test overflow
        let year = Year::from_number(2095).unwrap();
        assert!(year.add_years(10).is_err()); // Would exceed max year
    }

    #[test]
    fn test_subtract_years() {
        let year = Year::from_number(2025).unwrap();
        let past = year.subtract_years(5).unwrap();
        assert_eq!(past.year, 2020);

        // Test underflow
        let year = Year::from_number(1905).unwrap();
        assert!(year.subtract_years(10).is_err()); // Would be below min year
    }

    #[test]
    fn test_is_before() {
        let year1 = Year::from_number(2020).unwrap();
        let year2 = Year::from_number(2025).unwrap();

        assert!(year1.is_before(&year2));
        assert!(!year2.is_before(&year1));
        assert!(!year1.is_before(&year1));
    }

    #[test]
    fn test_is_after() {
        let year1 = Year::from_number(2020).unwrap();
        let year2 = Year::from_number(2025).unwrap();

        assert!(year2.is_after(&year1));
        assert!(!year1.is_after(&year2));
        assert!(!year1.is_after(&year1));
    }

    #[test]
    fn test_years_until() {
        let year1 = Year::from_number(2020).unwrap();
        let year2 = Year::from_number(2025).unwrap();

        assert_eq!(year1.years_until(&year2), 5);
        assert_eq!(year2.years_until(&year1), -5);
        assert_eq!(year1.years_until(&year1), 0);
    }

    #[test]
    fn test_years_since() {
        let year1 = Year::from_number(2020).unwrap();
        let year2 = Year::from_number(2025).unwrap();

        assert_eq!(year2.years_since(&year1), 5);
        assert_eq!(year1.years_since(&year2), -5);
        assert_eq!(year1.years_since(&year1), 0);
    }

    #[test]
    fn test_to_2digit_string() {
        let year = Year::from_number(2023).unwrap();
        assert_eq!(year.to_2digit_string(), "23");

        let year = Year::from_number(2005).unwrap();
        assert_eq!(year.to_2digit_string(), "05");
    }

    #[test]
    fn test_to_4digit_string() {
        let year = Year::from_number(2023).unwrap();
        assert_eq!(year.to_4digit_string(), "2023");

        let year = Year::from_number(1999).unwrap();
        assert_eq!(year.to_4digit_string(), "1999");
    }

    #[test]
    fn test_to_number() {
        let year = Year::from_number(2023).unwrap();
        assert_eq!(year.to_number(), 2023);

        let year = Year::from_2digit_number(99).unwrap();
        assert_eq!(year.to_number(), 1999);
    }

    #[test]
    fn test_to_2digit_text() {
        let year = Year::from_number(2023).unwrap();
        assert_eq!(year.to_2digit_text(), "23");

        let year = Year::from_number(2005).unwrap();
        assert_eq!(year.to_2digit_text(), "05");
    }

    #[test]
    fn test_to_4digit_text() {
        let year = Year::from_number(2023).unwrap();
        assert_eq!(year.to_4digit_text(), "2023");

        let year = Year::from_number(1999).unwrap();
        assert_eq!(year.to_4digit_text(), "1999");
    }

    #[test]
    fn test_is_leap_year() {
        let year = Year::from_number(2024).unwrap();
        assert!(year.is_leap_year());

        let year = Year::from_number(2023).unwrap();
        assert!(!year.is_leap_year());

        let year = Year::from_number(2000).unwrap();
        assert!(year.is_leap_year()); // Divisible by 400

        let year = Year::from_number(1900).unwrap();
        assert!(!year.is_leap_year()); // Divisible by 100 but not 400
    }

    #[test]
    fn test_days_in_year() {
        let year = Year::from_number(2024).unwrap();
        assert_eq!(year.days_in_year(), 366); // Leap year

        let year = Year::from_number(2023).unwrap();
        assert_eq!(year.days_in_year(), 365); // Regular year
    }

    #[test]
    fn test_days_in_month() {
        let year = Year::from_number(2024).unwrap(); // Leap year
        let feb = Month::from_number(2).unwrap();
        assert_eq!(year.days_in_month(&feb), 29); // February in leap year

        let year = Year::from_number(2023).unwrap(); // Regular year
        assert_eq!(year.days_in_month(&feb), 28); // February in regular year

        let jan = Month::from_number(1).unwrap();
        assert_eq!(year.days_in_month(&jan), 31); // January always has 31 days
    }

    #[test]
    fn test_year_start() {
        let year = Year::from_number(2023).unwrap();
        let start = year.year_start().unwrap();
        assert_eq!(start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    }

    #[test]
    fn test_year_end() {
        let year = Year::from_number(2023).unwrap();
        let end = year.year_end().unwrap();
        assert_eq!(end, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    }

    #[test]
    fn test_from_naive_date() {
        let date = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
        let year = Year::from_naive_date(&date).unwrap();
        assert_eq!(year.year, 2023);
    }

    #[test]
    fn test_from_datetime() {
        let datetime = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap()
            .and_hms_opt(14, 30, 0).unwrap()
            .and_utc();
        let year = Year::from_datetime(&datetime).unwrap();
        assert_eq!(year.year, 2023);
    }

    #[test]
    fn test_to_naive_date() {
        let year = Year::from_number(2023).unwrap();
        let month = Month::from_number(6).unwrap();
        let date = year.to_naive_date(&month, 15).unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2023, 6, 15).unwrap());

        // Invalid date
        let feb = Month::from_number(2).unwrap();
        assert!(year.to_naive_date(&feb, 30).is_err()); // February 30th doesn't exist
    }

    #[test]
    fn test_to_datetime_utc() {
        let year = Year::from_number(2023).unwrap();
        let month = Month::from_number(6).unwrap();
        let datetime = year.to_datetime_utc(&month, 15, 14, 30, 0).unwrap();

        let expected = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap()
            .and_hms_opt(14, 30, 0).unwrap()
            .and_utc();
        assert_eq!(datetime, expected);
    }

    #[test]
    fn test_to_datetime_with_timezone() {
        let year = Year::from_number(2023).unwrap();
        let month = Month::from_number(6).unwrap();
        let datetime = year.to_datetime_with_timezone(&month, 15, 14, 30, 0, &Local).unwrap();

        let expected = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap()
            .and_hms_opt(14, 30, 0).unwrap();
        assert_eq!(datetime.naive_local(), expected);
    }

    #[test]
    fn test_year_start_in_timezone() {
        let year = Year::from_number(2023).unwrap();
        let start = year.year_start_in_timezone(&Utc).unwrap();

        let expected = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
            .and_hms_opt(0, 0, 0).unwrap()
            .and_utc();
        assert_eq!(start, expected);
    }

    #[test]
    fn test_year_end_in_timezone() {
        let year = Year::from_number(2023).unwrap();
        let end = year.year_end_in_timezone(&Utc).unwrap();

        let expected = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()
            .and_hms_opt(23, 59, 59).unwrap()
            .and_utc();
        assert_eq!(end, expected);
    }

    #[test]
    fn test_quarter_start() {
        let year = Year::from_number(2023).unwrap();

        let q1_start = year.quarter_start(1).unwrap();
        assert_eq!(q1_start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());

        let q2_start = year.quarter_start(2).unwrap();
        assert_eq!(q2_start, NaiveDate::from_ymd_opt(2023, 4, 1).unwrap());

        let q3_start = year.quarter_start(3).unwrap();
        assert_eq!(q3_start, NaiveDate::from_ymd_opt(2023, 7, 1).unwrap());

        let q4_start = year.quarter_start(4).unwrap();
        assert_eq!(q4_start, NaiveDate::from_ymd_opt(2023, 10, 1).unwrap());

        // Invalid quarter
        assert!(year.quarter_start(5).is_err());
    }

    #[test]
    fn test_quarter_end() {
        let year = Year::from_number(2023).unwrap();

        let q1_end = year.quarter_end(1).unwrap();
        assert_eq!(q1_end, NaiveDate::from_ymd_opt(2023, 3, 31).unwrap());

        let q2_end = year.quarter_end(2).unwrap();
        assert_eq!(q2_end, NaiveDate::from_ymd_opt(2023, 6, 30).unwrap());

        let q3_end = year.quarter_end(3).unwrap();
        assert_eq!(q3_end, NaiveDate::from_ymd_opt(2023, 9, 30).unwrap());

        let q4_end = year.quarter_end(4).unwrap();
        assert_eq!(q4_end, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    }

    #[test]
    fn test_month_start() {
        let year = Year::from_number(2023).unwrap();
        let june = Month::from_number(6).unwrap();

        let month_start = year.month_start(&june).unwrap();
        assert_eq!(month_start, NaiveDate::from_ymd_opt(2023, 6, 1).unwrap());
    }

    #[test]
    fn test_month_end() {
        let year = Year::from_number(2023).unwrap();
        let june = Month::from_number(6).unwrap();
        let feb = Month::from_number(2).unwrap();

        let june_end = year.month_end(&june).unwrap();
        assert_eq!(june_end, NaiveDate::from_ymd_opt(2023, 6, 30).unwrap());

        let feb_end = year.month_end(&feb).unwrap();
        assert_eq!(feb_end, NaiveDate::from_ymd_opt(2023, 2, 28).unwrap()); // Non-leap year

        // Test leap year
        let leap_year = Year::from_number(2024).unwrap();
        let leap_feb_end = leap_year.month_end(&feb).unwrap();
        assert_eq!(leap_feb_end, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()); // Leap year
    }

    #[test]
    fn test_is_valid_date() {
        let year = Year::from_number(2023).unwrap(); // Non-leap year
        let leap_year = Year::from_number(2024).unwrap(); // Leap year
        let feb = Month::from_number(2).unwrap();
        let june = Month::from_number(6).unwrap();

        // Valid dates
        assert!(year.is_valid_date(&feb, 28));
        assert!(year.is_valid_date(&june, 30));
        assert!(leap_year.is_valid_date(&feb, 29)); // Valid in leap year

        // Invalid dates
        assert!(!year.is_valid_date(&feb, 29)); // Invalid in non-leap year
        assert!(!year.is_valid_date(&june, 31)); // June has only 30 days
        assert!(!year.is_valid_date(&feb, 0)); // Day 0 doesn't exist
    }

    #[test]
    fn test_month_ranges() {
        let year = Year::from_number(2023).unwrap();
        let month_ranges = year.month_ranges().unwrap();

        assert_eq!(month_ranges.len(), 12);

        // Check January
        let (jan_month, jan_start, jan_end) = &month_ranges[0];
        assert_eq!(jan_month.month, 1);
        assert_eq!(jan_start.day(), 1);
        assert_eq!(jan_end.day(), 31);

        // Check February (non-leap year)
        let (feb_month, feb_start, feb_end) = &month_ranges[1];
        assert_eq!(feb_month.month, 2);
        assert_eq!(feb_start.day(), 1);
        assert_eq!(feb_end.day(), 28); // Non-leap year
    }

    #[test]
    fn test_get_quarter() {
        let year = Year::from_number(2023).unwrap();

        let jan = Month::from_number(1).unwrap();
        let apr = Month::from_number(4).unwrap();
        let jul = Month::from_number(7).unwrap();
        let oct = Month::from_number(10).unwrap();

        assert_eq!(year.get_quarter(&jan), 1);
        assert_eq!(year.get_quarter(&apr), 2);
        assert_eq!(year.get_quarter(&jul), 3);
        assert_eq!(year.get_quarter(&oct), 4);
    }

    #[test]
    fn test_get_quarter_months() {
        let year = Year::from_number(2023).unwrap();

        let q1_months = year.get_quarter_months(1).unwrap();
        assert_eq!(q1_months.len(), 3);
        assert_eq!(q1_months[0].month, 1); // January
        assert_eq!(q1_months[1].month, 2); // February
        assert_eq!(q1_months[2].month, 3); // March

        let q4_months = year.get_quarter_months(4).unwrap();
        assert_eq!(q4_months.len(), 3);
        assert_eq!(q4_months[0].month, 10); // October
        assert_eq!(q4_months[1].month, 11); // November
        assert_eq!(q4_months[2].month, 12); // December

        // Invalid quarter
        assert!(year.get_quarter_months(5).is_err());
    }

    #[test]
    fn test_is_valid() {
        // Works with i32
        assert!(Year::is_valid(2023i32));
        assert!(!Year::is_valid(1800i32));

        // Works with &str - various formats
        assert!(Year::is_valid("2023"));
        assert!(Year::is_valid("23")); // 2-digit

        // Works with String
        assert!(Year::is_valid(String::from("2024")));

        // Invalid cases
        assert!(!Year::is_valid("invalid"));
        assert!(!Year::is_valid("1800"));
        assert!(!Year::is_valid("2200"));
    }
}