#[cfg(test)]
mod tests {
    use crate::utils::path::{ValidatedPath, PathValidatable, PathFromInput, PathLike};
    use std::path::PathBuf;

    #[test]
    fn test_path_validatable_str() {
        // String validation
        assert!("/valid/path".is_valid_path());
        assert!(!"".is_valid_path()); // Empty path
        assert!(!"path/with\0null".is_valid_path()); // Contains null character
    }

    #[test]
    fn test_path_validatable_pathbuf() {
        // PathBuf validation
        let path = PathBuf::from("/valid/path");
        assert!(path.is_valid_path());
    }

    #[test]
    fn test_path_validatable_characters() {
        // Character validation
        assert!("valid/path/file.txt".has_valid_characters());
        assert!(!"path/with<bracket".has_valid_characters());
    }

    #[test]
    fn test_path_validatable_length() {
        // Length validation
        let long_path = "a".repeat(513);
        assert!(!long_path.has_valid_length());
    }

    #[test]
    fn test_path_from_input_str() {
        // String parsing
        let path = "/valid/path".parse_path().unwrap();
        assert_eq!(path.as_path().to_str().unwrap(), "/valid/path");
    }

    #[test]
    fn test_path_from_input_pathbuf() {
        // PathBuf parsing
        let pathbuf = PathBuf::from("/another/path");
        let path = pathbuf.parse_path().unwrap();
        assert_eq!(path.into_path_buf(), PathBuf::from("/another/path"));
    }

    #[test]
    fn test_path_from_input_invalid() {
        // Invalid cases
        assert!("".parse_path().is_err()); // Empty path
        assert!("path/with\0null".parse_path().is_err()); // Invalid characters
    }

    #[test]
    fn test_validated_path_new_str() {
        // From string
        let path = ValidatedPath::new("/valid/path").unwrap();
        assert_eq!(path.to_string_lossy(), "/valid/path");
    }

    #[test]
    fn test_validated_path_new_pathbuf() {
        // From PathBuf
        let pathbuf = PathBuf::from("/another/path");
        let path = ValidatedPath::new(pathbuf).unwrap();
        assert_eq!(path.to_string_lossy(), "/another/path");
    }

    #[test]
    fn test_validated_path_new_invalid() {
        // Invalid cases
        assert!(ValidatedPath::new("").is_err()); // Empty path
        assert!(ValidatedPath::new("path/with\0null").is_err()); // Invalid characters
    }

    #[test]
    fn test_path_like_pathbuf() {
        // Working with PathBuf
        let pathbuf = PathBuf::from("/valid/path");
        assert_eq!(pathbuf.as_string().unwrap(), "/valid/path");
        assert!(pathbuf.validate().is_ok());
    }

    #[test]
    fn test_path_like_str() {
        // Working with &str
        let path_str = "/another/path";
        assert_eq!(path_str.as_string().unwrap(), "/another/path");
        assert!(path_str.validate().is_ok());
    }

    #[test]
    fn test_path_like_string() {
        // Working with String
        let path_string = String::from("/string/path");
        assert_eq!(path_string.as_string().unwrap(), "/string/path");
        assert!(path_string.validate().is_ok());
    }

    #[test]
    fn test_validated_path_is_valid() {
        assert!(ValidatedPath::is_valid("/valid/path"));
        assert!(!ValidatedPath::is_valid(""));
        assert!(!ValidatedPath::is_valid("path/with\0null"));

        // Works with different types
        assert!(ValidatedPath::is_valid(String::from("/string/path")));
        assert!(ValidatedPath::is_valid(PathBuf::from("/pathbuf/path")));
    }

    #[test]
    fn test_validated_path_validate() {
        assert!(ValidatedPath::validate("/valid/path").is_ok());
        assert!(ValidatedPath::validate("").is_err());
        assert!(ValidatedPath::validate("path/with\0null").is_err());

        // Test path length validation
        let long_path = "a".repeat(513);
        assert!(ValidatedPath::validate(long_path).is_err());
    }

    #[test]
    fn test_validated_path_methods() {
        let path = ValidatedPath::new("/valid/path/file.txt").unwrap();
        
        // Test basic methods
        assert_eq!(path.as_path().to_str().unwrap(), "/valid/path/file.txt");
        assert_eq!(path.file_name().unwrap().to_str().unwrap(), "file.txt");
        assert_eq!(path.extension().unwrap().to_str().unwrap(), "txt");
        assert_eq!(path.parent().unwrap().to_str().unwrap(), "/valid/path");
    }

    #[test]
    fn test_validated_path_join() {
        let path = ValidatedPath::new("/valid/path").unwrap();
        let joined = path.join("file.txt").unwrap();
        assert_eq!(joined.to_string_lossy(), "/valid/path/file.txt");
    }

    #[test]
    fn test_validated_path_absolute_relative() {
        let abs_path = ValidatedPath::new("/absolute/path").unwrap();
        assert!(abs_path.is_absolute());
        assert!(!abs_path.is_relative());

        let rel_path = ValidatedPath::new("relative/path").unwrap();
        assert!(!rel_path.is_absolute());
        assert!(rel_path.is_relative());
    }

    #[test]
    fn test_path_validatable_implementations() {
        // Test all PathValidatable implementations
        assert!("/valid/path".is_valid_path());
        assert!(String::from("/valid/path").is_valid_path());
        assert!(PathBuf::from("/valid/path").is_valid_path());
        
        // Test invalid paths
        assert!(!"".is_valid_path());
        assert!(!String::from("").is_valid_path());
        assert!(!PathBuf::from("").is_valid_path());
    }

    #[test]
    fn test_path_like_filesystem_methods() {
        // These tests don't actually check filesystem state, just that methods compile
        let path_str = "/some/path";
        let _ = path_str.exists();
        let _ = path_str.is_file();
        let _ = path_str.is_dir();
        
        let pathbuf = PathBuf::from("/some/path");
        let _ = pathbuf.exists();
        let _ = pathbuf.is_file();
        let _ = pathbuf.is_dir();
    }

    #[test]
    fn test_path_error_cases() {
        // Test various error conditions
        assert!("".parse_path().is_err());
        assert!("path/with\0null".parse_path().is_err());
        
        let long_path = "a".repeat(513);
        assert!(long_path.parse_path().is_err());
        
        // Test with invalid characters
        assert!("path/with<bracket".parse_path().is_err());
        assert!("path/with>bracket".parse_path().is_err());
        assert!("path/with|pipe".parse_path().is_err());
    }

    #[test]
    fn test_path_from_input_implementations() {
        // Test all PathFromInput implementations
        let str_path = "/valid/path";
        assert!(str_path.parse_path().is_ok());
        
        let string_path = String::from("/valid/path");
        assert!(string_path.parse_path().is_ok());
        
        let pathbuf_path = PathBuf::from("/valid/path");
        assert!(pathbuf_path.parse_path().is_ok());
    }
}