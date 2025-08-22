#[cfg(test)]
mod tests {
    use crate::utils::Month;

    #[test]
    fn test_all_months() {
        let months = Month::all_months();
        assert_eq!(months.len(), 12);
        assert_eq!(months[0].month, 1);
        assert_eq!(months[0].name_en, "January");
        assert_eq!(months[11].month, 12);
        assert_eq!(months[11].name_en, "December");
    }

    #[test]
    fn test_next_month() {
        let january = Month::from_number(1).unwrap();
        let february = january.next();
        assert_eq!(february.month, 2);
        assert_eq!(february.name_en, "February");

        // Test wrapping
        let december = Month::from_number(12).unwrap();
        let january_again = december.next();
        assert_eq!(january_again.month, 1);
        assert_eq!(january_again.name_en, "January");
    }

    #[test]
    fn test_previous_month() {
        let february = Month::from_number(2).unwrap();
        let january = february.previous();
        assert_eq!(january.month, 1);
        assert_eq!(january.name_en, "January");

        // Test wrapping
        let january = Month::from_number(1).unwrap();
        let december = january.previous();
        assert_eq!(december.month, 12);
        assert_eq!(december.name_en, "December");
    }

    #[test]
    fn test_is_before() {
        let january = Month::from_number(1).unwrap();
        let march = Month::from_number(3).unwrap();

        assert!(january.is_before(&march));
        assert!(!march.is_before(&january));
        assert!(!january.is_before(&january));
    }

    #[test]
    fn test_is_after() {
        let january = Month::from_number(1).unwrap();
        let march = Month::from_number(3).unwrap();

        assert!(march.is_after(&january));
        assert!(!january.is_after(&march));
        assert!(!january.is_after(&january));
    }

    #[test]
    fn test_months_until() {
        let january = Month::from_number(1).unwrap();
        let march = Month::from_number(3).unwrap();
        let november = Month::from_number(11).unwrap();

        assert_eq!(january.months_until(&march), 2);
        assert_eq!(march.months_until(&january), -2);
        assert_eq!(january.months_until(&november), 10);
        assert_eq!(january.months_until(&january), 0);
    }

    #[test]
    fn test_months_since() {
        let january = Month::from_number(1).unwrap();
        let march = Month::from_number(3).unwrap();

        assert_eq!(march.months_since(&january), 2);
        assert_eq!(january.months_since(&march), -2);
        assert_eq!(january.months_since(&january), 0);
    }

    #[test]
    fn test_to_zero_padded_string() {
        let january = Month::from_number(1).unwrap();
        let december = Month::from_number(12).unwrap();

        assert_eq!(january.to_zero_padded_string(), "01");
        assert_eq!(december.to_zero_padded_string(), "12");
    }

    #[test]
    fn test_to_number_string() {
        let january = Month::from_number(1).unwrap();
        let december = Month::from_number(12).unwrap();

        assert_eq!(january.to_number_string(), "1");
        assert_eq!(december.to_number_string(), "12");
    }

    #[test]
    fn test_to_number() {
        let january = Month::from_number(1).unwrap();
        assert_eq!(january.to_number(), 1);

        let december = Month::from_number(12).unwrap();
        assert_eq!(december.to_number(), 12);

        let july = Month::from_english_name("July").unwrap();
        assert_eq!(july.to_number(), 7);
    }

    #[test]
    fn test_to_text() {
        let january = Month::from_number(1).unwrap();
        assert_eq!(january.to_text(), "01");

        let december = Month::from_number(12).unwrap();
        assert_eq!(december.to_text(), "12");

        let may = Month::from_english_name("May").unwrap();
        assert_eq!(may.to_text(), "05");
    }

    #[test]
    fn test_to_ptbr() {
        let january = Month::from_number(1).unwrap();
        assert_eq!(january.to_ptbr(), "Janeiro");

        let february = Month::from_number(2).unwrap();
        assert_eq!(february.to_ptbr(), "Fevereiro");

        let march = Month::from_abbreviation("Mar").unwrap();
        assert_eq!(march.to_ptbr(), "Março");

        let december = Month::from_english_name("December").unwrap();
        assert_eq!(december.to_ptbr(), "Dezembro");
    }

    #[test]
    fn test_to_en() {
        let january = Month::from_number(1).unwrap();
        assert_eq!(january.to_en(), "January");

        let february = Month::from_number(2).unwrap();
        assert_eq!(february.to_en(), "February");

        let march = Month::from_portuguese_name("Março").unwrap();
        assert_eq!(march.to_en(), "March");

        let december = Month::from_abbreviation("Dez").unwrap();
        assert_eq!(december.to_en(), "December");
    }

    #[test]
    fn test_to_short() {
        let january = Month::from_number(1).unwrap();
        assert_eq!(january.to_short(), "Jan");

        let february = Month::from_number(2).unwrap();
        assert_eq!(february.to_short(), "Fev");

        let march = Month::from_english_name("March").unwrap();
        assert_eq!(march.to_short(), "Mar");

        let august = Month::from_portuguese_name("Agosto").unwrap();
        assert_eq!(august.to_short(), "Ago");

        let december = Month::from_text("12").unwrap();
        assert_eq!(december.to_short(), "Dez");
    }

    #[test]
    fn test_from_generic() {
        // Parse from number
        let jan = Month::from(1u8).unwrap();
        assert_eq!(jan.month, 1);

        // Parse from zero-padded text
        let feb = Month::from("02").unwrap();
        assert_eq!(feb.month, 2);

        // Parse from number string
        let mar = Month::from("3").unwrap();
        assert_eq!(mar.month, 3);

        // Parse from English name (case-insensitive)
        let apr = Month::from("April").unwrap();
        assert_eq!(apr.month, 4);

        let may = Month::from("MAY").unwrap();
        assert_eq!(may.month, 5);

        // Parse from Portuguese name (case-insensitive)
        let jun = Month::from("Junho").unwrap();
        assert_eq!(jun.month, 6);

        let jul = Month::from("JULHO").unwrap();
        assert_eq!(jul.month, 7);

        // Parse from abbreviation (case-insensitive)
        let aug = Month::from("Ago").unwrap();
        assert_eq!(aug.month, 8);

        let sep = Month::from("SET").unwrap();
        assert_eq!(sep.month, 9);

        // Invalid cases
        assert!(Month::from("invalid").is_err());
        assert!(Month::from("13").is_err());
        assert!(Month::from(0u8).is_err());
    }

    #[test]
    fn test_from_number() {
        let january = Month::from_number(1).unwrap();
        assert_eq!(january.month, 1);
        assert_eq!(january.name_en, "January");

        let december = Month::from_number(12).unwrap();
        assert_eq!(december.month, 12);
        assert_eq!(december.name_en, "December");

        // Invalid cases
        assert!(Month::from_number(0).is_err());
        assert!(Month::from_number(13).is_err());
    }

    #[test]
    fn test_from_text() {
        let january = Month::from_text("01").unwrap();
        assert_eq!(january.month, 1);
        assert_eq!(january.name_en, "January");

        let december = Month::from_text("12").unwrap();
        assert_eq!(december.month, 12);

        // Invalid cases
        assert!(Month::from_text("00").is_err());
        assert!(Month::from_text("13").is_err());
        assert!(Month::from_text("1").is_err()); // Must be zero-padded
    }

    #[test]
    fn test_from_english_name() {
        let january = Month::from_english_name("January").unwrap();
        assert_eq!(january.month, 1);

        // Case insensitive
        let february = Month::from_english_name("february").unwrap();
        assert_eq!(february.month, 2);

        let march = Month::from_english_name("MARCH").unwrap();
        assert_eq!(march.month, 3);

        // Invalid cases
        assert!(Month::from_english_name("Invalid").is_err());
        assert!(Month::from_english_name("Janeiro").is_err()); // Portuguese name
    }

    #[test]
    fn test_from_portuguese_name() {
        let january = Month::from_portuguese_name("Janeiro").unwrap();
        assert_eq!(january.month, 1);

        // Case insensitive
        let february = Month::from_portuguese_name("fevereiro").unwrap();
        assert_eq!(february.month, 2);

        let march = Month::from_portuguese_name("MARÇO").unwrap();
        assert_eq!(march.month, 3);

        // Invalid cases
        assert!(Month::from_portuguese_name("Invalid").is_err());
        assert!(Month::from_portuguese_name("January").is_err()); // English name
    }

    #[test]
    fn test_from_abbreviation() {
        let january = Month::from_abbreviation("Jan").unwrap();
        assert_eq!(january.month, 1);

        // Case insensitive
        let february = Month::from_abbreviation("Fev").unwrap();
        assert_eq!(february.month, 2);

        let march = Month::from_abbreviation("MAR").unwrap();
        assert_eq!(march.month, 3);

        // Invalid cases
        assert!(Month::from_abbreviation("Invalid").is_err());
        assert!(Month::from_abbreviation("January").is_err()); // Full name, not abbreviation
    }

    #[test]
    fn test_is_valid() {
        // Works with u8
        assert!(Month::is_valid(5u8));
        assert!(!Month::is_valid(13u8));

        // Works with &str - various formats
        assert!(Month::is_valid("January"));
        assert!(Month::is_valid("01"));
        assert!(Month::is_valid("Jan"));
        assert!(Month::is_valid("Janeiro"));
        assert!(Month::is_valid("5"));

        // Case insensitive
        assert!(Month::is_valid("january"));
        assert!(Month::is_valid("FEBRUARY"));

        // Works with String
        assert!(Month::is_valid(String::from("March")));

        // Invalid cases
        assert!(!Month::is_valid("invalid"));
        assert!(!Month::is_valid("13"));
        assert!(!Month::is_valid("0"));
    }

    #[test]
    fn test_month_validatable_trait() {
        use crate::utils::time::month::MonthValidatable;

        // u8 validation
        assert!(5u8.is_valid_month());
        assert!(!13u8.is_valid_month());

        // str validation
        assert!("January".is_valid_month());
        assert!("01".is_valid_month());
        assert!(!"invalid".is_valid_month());

        // String validation
        assert!(String::from("February").is_valid_month());
    }

    #[test]
    fn test_month_from_input_trait() {
        use crate::utils::time::month::MonthFromInput;

        // u8 parsing
        assert!(1u8.parse_month().is_ok());
        assert!(13u8.parse_month().is_err());

        // str parsing
        assert!("January".parse_month().is_ok());
        assert!("01".parse_month().is_ok());
        assert!("invalid".parse_month().is_err());

        // String parsing
        assert!(String::from("February").parse_month().is_ok());
    }
}