#[cfg(test)]
mod tests {
    use crate::utils::time::Duration;

    #[test]
    fn test_duration_constructors() {
        let d1 = Duration::from_hours(2);
        assert_eq!(d1.total_hours(), 2);
        assert_eq!(d1.total_minutes(), 120);
        assert_eq!(d1.total_seconds(), 7200);

        let d2 = Duration::from_minutes(30);
        assert_eq!(d2.total_minutes(), 30);
        assert_eq!(d2.total_seconds(), 1800);

        let d3 = Duration::from_seconds(45);
        assert_eq!(d3.total_seconds(), 45);

        let d4 = Duration::from_components(1, 30, 45, 500, 0);
        assert_eq!(d4.hours(), 1);
        assert_eq!(d4.minutes(), 30);
        assert_eq!(d4.seconds(), 45);
        assert_eq!(d4.millis(), 500);
    }

    #[test]
    fn test_duration_arithmetic() {
        let d1 = Duration::from_hours(1);
        let d2 = Duration::from_minutes(30);
        
        let sum = d1.add(&d2).unwrap();
        assert_eq!(sum.total_minutes(), 90);

        let diff = d1.subtract(&d2).unwrap();
        assert_eq!(diff.total_minutes(), 30);

        let doubled = d1.multiply(2).unwrap();
        assert_eq!(doubled.total_hours(), 2);

        let halved = d1.divide(2).unwrap();
        assert_eq!(halved.total_minutes(), 30);
    }

    #[test]
    fn test_duration_formatting() {
        let d1 = Duration::from_components(2, 30, 45, 0, 0);
        assert_eq!(d1.to_readable(), "2h 30m 45s");
        assert_eq!(d1.to_hms(), "02:30:45");

        let d2 = Duration::from_seconds(75);
        assert_eq!(d2.to_readable(), "1m 15s");

        let d3 = Duration::from_millis(1500);
        assert_eq!(d3.to_readable(), "1.500s");
    }

    #[test]
    fn test_duration_parsing() {
        let d1 = Duration::parse_duration_string("2:30:45").unwrap();
        assert_eq!(d1.total_hours(), 2);
        assert_eq!(d1.minutes(), 30);
        assert_eq!(d1.seconds(), 45);

        let d2 = Duration::parse_duration_string("1h30m45s").unwrap();
        assert_eq!(d2.total_hours(), 1);
        assert_eq!(d2.minutes(), 30);
        assert_eq!(d2.seconds(), 45);
    }

    #[test]
    fn test_duration_validation() {
        assert!(Duration::is_valid(3600u64)); // 1 hour in seconds
        assert!(Duration::is_valid("1h30m"));
        assert!(Duration::is_valid("2:30:45"));
    }

    #[test]
    fn test_duration_zero() {
        let zero = Duration::zero();
        assert!(zero.is_zero());
        assert_eq!(zero.total_nanos(), 0);
    }

    #[test]
    fn test_duration_comparison() {
        let d1 = Duration::from_hours(2);
        let d2 = Duration::from_minutes(90);
        let d3 = Duration::from_hours(3);

        assert!(d3.is_longer_than(&d1));
        assert!(d2.is_shorter_than(&d1));
        assert!(!d1.is_longer_than(&d3));
    }

    #[test]
    fn test_duration_conversion() {
        let d = Duration::from_components(1, 30, 45, 500, 0);
        
        let std_duration = d.to_std_duration();
        assert_eq!(std_duration.as_secs(), 5445); // 1*3600 + 30*60 + 45
        assert_eq!(std_duration.subsec_millis(), 500);

        let chrono_duration = d.to_chrono_duration();
        assert_eq!(chrono_duration.num_seconds(), 5445);
    }
}