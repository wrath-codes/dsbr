# Path Utilities

The ArrowSUS path utilities provide a comprehensive, type-safe, and performant way to work with file system paths. The module offers robust validation, error handling, and cross-platform compatibility while maintaining excellent performance through optimized data structures.

## Overview

The path utilities are built around several key components:

- **PathError**: Comprehensive error handling for path operations
- **PathValidatable**: Trait for validating paths across different types
- **PathFromInput**: Trait for parsing paths from various input types
- **ValidatedPath**: A wrapper that ensures paths meet validation criteria
- **PathLike**: Enhanced trait for unified path operations

## Core Features

### ✅ **Comprehensive Validation**
- Path length validation (max 512 characters)
- Invalid character detection (Windows-compatible)
- Empty path detection
- UTF-8 validation

### ✅ **Cross-Platform Compatibility**
- Uses most restrictive validation rules (Windows-based)
- Handles different path separators
- Proper Unicode support

### ✅ **Performance Optimized**
- Lazy-loaded invalid character sets using `LazyLock`
- Efficient character validation with `DashSet`
- Zero-copy operations where possible

### ✅ **Type Safety**
- Compile-time guarantees for validated paths
- Rich error types with detailed messages
- Generic traits for flexible usage

## Quick Start

```rust
use arrow_sus_shared::utils::path::{PathValidatable, ValidatedPath, PathLike};
use std::path::PathBuf;

// Basic validation
assert!("/valid/path".is_valid_path());
assert!(!"".is_valid_path()); // Empty paths are invalid

// Create validated paths
let path = ValidatedPath::new("/home/user/documents").unwrap();
println!("Path: {}", path.to_string_lossy());

// Work with different path types uniformly
let pathbuf = PathBuf::from("/etc/config");
assert!(pathbuf.validate().is_ok());
assert_eq!(pathbuf.as_string().unwrap(), "/etc/config");
```

## Error Handling

The `PathError` enum provides detailed error information for different failure scenarios:

```rust
use arrow_sus_shared::utils::path::{PathError, ValidatedPath};

// Different error types
match ValidatedPath::new("") {
    Err(e) => {
        // Will be PathError::EmptyPath
        println!("Error: {}", e);
    }
    Ok(_) => unreachable!(),
}

// Path too long
let long_path = "a".repeat(600);
match ValidatedPath::new(long_path) {
    Err(e) => {
        // Will be PathError::PathTooLong(600, 512)
        println!("Error: {}", e);
    }
    Ok(_) => unreachable!(),
}

// Invalid characters
match ValidatedPath::new("path/with<invalid>chars") {
    Err(e) => {
        // Will be PathError::InvalidCharacters
        println!("Error: {}", e);
    }
    Ok(_) => unreachable!(),
}
```

### Error Types

| Error Type | Description | Example |
|------------|-------------|---------|
| `InvalidPath` | General path validation failure | `PathError::invalid_path("bad path")` |
| `InvalidUtf8` | Path contains invalid UTF-8 | `PathError::invalid_utf8("path")` |
| `EmptyPath` | Path is empty | `PathError::empty_path()` |
| `PathTooLong` | Path exceeds maximum length | `PathError::path_too_long(600, 512)` |
| `InvalidCharacters` | Path contains forbidden characters | `PathError::invalid_characters("path<>")` |
| `CannotConvertPath` | Path conversion failed | `PathError::cannot_convert_path("reason")` |
| `CannotParsePath` | Path parsing failed | `PathError::cannot_parse_path("reason")` |
| `PathNotFound` | Path does not exist | `PathError::path_not_found("/missing")` |
| `NotAbsolute` | Path is not absolute when required | `PathError::not_absolute("relative/path")` |
| `NotRelative` | Path is not relative when required | `PathError::not_relative("/absolute/path")` |

## Validation

### PathValidatable Trait

The `PathValidatable` trait provides validation methods for any path-like type:

```rust
use arrow_sus_shared::utils::path::PathValidatable;
use std::path::{Path, PathBuf};

// String validation
assert!("/valid/path".is_valid_path());
assert!("relative/path".is_valid_path());
assert!(!"".is_valid_path()); // Empty
assert!(!"path/with\0null".is_valid_path()); // Invalid character

// PathBuf validation
let pathbuf = PathBuf::from("/home/user");
assert!(pathbuf.is_valid_path());
assert!(pathbuf.is_absolute_path());
assert!(!pathbuf.is_relative_path());

// Character validation
assert!("valid/path/file.txt".has_valid_characters());
assert!(!"path/with<bracket".has_valid_characters());
assert!(!"path/with>bracket".has_valid_characters());
assert!(!"path/with:colon".has_valid_characters());
assert!(!"path/with\"quote".has_valid_characters());
assert!(!"path/with|pipe".has_valid_characters());
assert!(!"path/with?question".has_valid_characters());
assert!(!"path/with*asterisk".has_valid_characters());

// Length validation
let long_path = "a".repeat(513);
assert!(!long_path.has_valid_length());
let valid_path = "a".repeat(512);
assert!(valid_path.has_valid_length());
```

### Validation Methods

| Method | Description | Returns |
|--------|-------------|---------|
| `is_valid_path()` | Complete path validation | `bool` |
| `is_absolute_path()` | Check if path is absolute | `bool` |
| `is_relative_path()` | Check if path is relative | `bool` |
| `has_valid_length()` | Check path length (≤512 chars) | `bool` |
| `has_valid_characters()` | Check for invalid characters | `bool` |

### Supported Types

The `PathValidatable` trait is implemented for:
- `str` and `&str`
- `String` and `&String`
- `Path` and `&Path`
- `PathBuf`
- `OsStr` and `&OsStr`
- `OsString` and `&OsString`

## Path Parsing

### PathFromInput Trait

Convert various input types into validated paths:

```rust
use arrow_sus_shared::utils::path::{PathFromInput, ValidatedPath};
use std::path::PathBuf;

// String parsing
let path = "/valid/path".parse_path().unwrap();
assert_eq!(path.as_path().to_str().unwrap(), "/valid/path");

// PathBuf parsing
let pathbuf = PathBuf::from("/another/path");
let path = pathbuf.parse_path().unwrap();
assert_eq!(path.into_path_buf(), PathBuf::from("/another/path"));

// Error handling
assert!("".parse_path().is_err()); // Empty path
assert!("path/with\0null".parse_path().is_err()); // Invalid characters

// Generic parsing
let path = ValidatedPath::new("/home/user").unwrap();
assert_eq!(path.to_string_lossy(), "/home/user");
```

## ValidatedPath

The `ValidatedPath` struct ensures that paths meet all validation criteria:

```rust
use arrow_sus_shared::utils::path::ValidatedPath;
use std::path::PathBuf;

// Create validated paths
let path = ValidatedPath::new("/home/user/documents").unwrap();

// Path operations
assert!(path.is_absolute());
assert!(!path.is_relative());
assert_eq!(path.file_name().unwrap(), "documents");
assert_eq!(path.parent().unwrap(), Path::new("/home/user"));

// Join paths (with validation)
let config_path = path.join("config.toml").unwrap();
assert_eq!(config_path.to_string_lossy(), "/home/user/documents/config.toml");

// Convert back to standard types
let pathbuf: PathBuf = path.into_path_buf();
let path_ref: &Path = path.as_path();
```

### ValidatedPath Methods

| Method | Description | Returns |
|--------|-------------|---------|
| `new(input)` | Create from any path-like input | `Result<ValidatedPath>` |
| `into_path_buf()` | Convert to PathBuf | `PathBuf` |
| `as_path()` | Get Path reference | `&Path` |
| `is_absolute()` | Check if absolute | `bool` |
| `is_relative()` | Check if relative | `bool` |
| `file_name()` | Get filename component | `Option<&OsStr>` |
| `parent()` | Get parent directory | `Option<&Path>` |
| `extension()` | Get file extension | `Option<&OsStr>` |
| `join(path)` | Join with another path | `Result<ValidatedPath>` |
| `to_string_lossy()` | Convert to string | `Cow<str>` |

### Static Validation Methods

```rust
use arrow_sus_shared::utils::path::ValidatedPath;
use std::path::PathBuf;

// Quick validation without creating instance
assert!(ValidatedPath::is_valid("/valid/path"));
assert!(!ValidatedPath::is_valid(""));
assert!(!ValidatedPath::is_valid("path/with\0null"));

// Works with different types
assert!(ValidatedPath::is_valid(String::from("/string/path")));
assert!(ValidatedPath::is_valid(PathBuf::from("/pathbuf/path")));

// Detailed validation with error info
assert!(ValidatedPath::validate("/valid/path").is_ok());
assert!(ValidatedPath::validate("").is_err());
assert!(ValidatedPath::validate("path/with\0null").is_err());

// Test path length validation
let long_path = "a".repeat(513);
assert!(ValidatedPath::validate(long_path).is_err());
```

## PathLike Trait

The `PathLike` trait provides a unified interface for working with different path types:

```rust
use arrow_sus_shared::utils::path::PathLike;
use std::path::PathBuf;

// Works with PathBuf
let pathbuf = PathBuf::from("/valid/path");
assert_eq!(pathbuf.as_string().unwrap(), "/valid/path");
assert!(pathbuf.validate().is_ok());
assert!(pathbuf.exists()); // Check filesystem
assert!(pathbuf.is_dir());

// Works with &str
let path_str = "/another/path";
assert_eq!(path_str.as_string().unwrap(), "/another/path");
assert!(path_str.validate().is_ok());

// Works with String
let path_string = String::from("/string/path");
assert_eq!(path_string.as_string().unwrap(), "/string/path");
assert!(path_string.validate().is_ok());

// Filesystem operations
if path_str.exists() {
    if path_str.is_file() {
        println!("It's a file");
    } else if path_str.is_dir() {
        println!("It's a directory");
    }
}

// Get canonical path
let canonical = pathbuf.canonicalize().unwrap();
println!("Canonical path: {}", canonical.display());
```

### PathLike Methods

| Method | Description | Returns |
|--------|-------------|---------|
| `as_path()` | Get Path reference | `Result<&Path>` |
| `to_path_buf()` | Convert to PathBuf | `Result<PathBuf>` |
| `as_string()` | Convert to String | `Result<String>` |
| `validate()` | Validate the path | `Result<()>` |
| `exists()` | Check if path exists | `bool` |
| `is_file()` | Check if path is a file | `bool` |
| `is_dir()` | Check if path is a directory | `bool` |
| `canonicalize()` | Get canonical absolute path | `Result<PathBuf>` |

## Performance Considerations

The path utilities are designed for high performance:

### Optimized Data Structures

```rust
// Invalid characters are stored in a high-performance hash set
use std::sync::LazyLock;
use dashmap::DashSet;

pub static INVALID_PATH_CHARS: LazyLock<DashSet<char>> = LazyLock::new(|| {
    let chars = DashSet::new();
    // Populated with invalid characters: < > : " | ? * and control chars
    chars
});
```

### Efficient Validation

- **Lazy Loading**: Invalid character set is only initialized when first used
- **Fast Lookups**: `DashSet` provides O(1) character validation
- **Zero-Copy**: String operations avoid unnecessary allocations where possible
- **Early Returns**: Validation stops at first failure

### Benchmarking

The validation is designed to handle thousands of path validations efficiently:

```rust
// Example: Validating 1000 paths should complete in microseconds
let paths: Vec<&str> = vec!["/valid/path"; 1000];
let start = std::time::Instant::now();

for path in paths {
    assert!(path.is_valid_path());
}

let duration = start.elapsed();
// Should complete in well under 1ms
assert!(duration.as_millis() < 1);
```

## Configuration

### Path Length Limit

The maximum path length is configurable via a constant:

```rust
/// Maximum allowed path length (platform-dependent, using conservative value)
pub const MAX_PATH_LENGTH: usize = 512;
```

This conservative limit ensures compatibility across different platforms:
- **Windows**: 260 characters (legacy), 32,767 (with long path support)
- **Linux**: 4096 characters
- **macOS**: 1024 characters

### Invalid Characters

The system uses Windows-compatible restrictions for maximum portability:

- `<` `>` `:` `"` `|` `?` `*` (Windows reserved)
- Control characters (0-31)
- Null character (`\0`)

## Best Practices

### 1. Use ValidatedPath for Guaranteed Safety

```rust
use arrow_sus_shared::utils::path::ValidatedPath;

// Good: Validation happens once at creation
let path = ValidatedPath::new(user_input)?;
process_path(path);

// Less ideal: Repeated validation
if user_input.is_valid_path() {
    let pathbuf = PathBuf::from(user_input);
    if pathbuf.is_valid_path() { // Redundant check
        process_pathbuf(pathbuf);
    }
}
```

### 2. Handle Errors Appropriately

```rust
use arrow_sus_shared::utils::path::{ValidatedPath, PathError};

match ValidatedPath::new(user_input) {
    Ok(path) => {
        // Use validated path
        process_file(&path);
    }
    Err(e) => {
        match e.downcast_ref::<PathError>() {
            Some(PathError::EmptyPath) => {
                eprintln!("Please provide a path");
            }
            Some(PathError::PathTooLong(actual, max)) => {
                eprintln!("Path too long: {} characters (max: {})", actual, max);
            }
            Some(PathError::InvalidCharacters(msg)) => {
                eprintln!("Invalid characters in path: {}", msg);
            }
            _ => {
                eprintln!("Path validation failed: {}", e);
            }
        }
    }
}
```

### 3. Leverage Generic Traits

```rust
use arrow_sus_shared::utils::path::{PathValidatable, PathLike};

// Generic function that works with any path type
fn process_any_path<P>(path: P) -> Result<String, Box<dyn std::error::Error>>
where
    P: PathLike + PathValidatable,
{
    // Validate first
    if !path.is_valid_path() {
        return Err("Invalid path".into());
    }
    
    // Use unified interface
    let path_string = path.as_string()?;
    
    // Check filesystem
    if path.exists() {
        Ok(format!("Found: {}", path_string))
    } else {
        Ok(format!("Not found: {}", path_string))
    }
}

// Works with any path type
let result1 = process_any_path("/some/path")?;
let result2 = process_any_path(PathBuf::from("/other/path"))?;
let result3 = process_any_path(String::from("/string/path"))?;
```

### 4. Efficient Batch Validation

```rust
use arrow_sus_shared::utils::path::PathValidatable;

// Efficient batch processing
fn validate_paths(paths: &[&str]) -> Vec<&str> {
    paths
        .iter()
        .filter(|path| path.is_valid_path())
        .copied()
        .collect()
}

// Or with detailed error reporting
fn validate_paths_detailed(paths: &[&str]) -> (Vec<&str>, Vec<(&str, String)>) {
    let mut valid = Vec::new();
    let mut invalid = Vec::new();
    
    for &path in paths {
        if path.is_valid_path() {
            valid.push(path);
        } else {
            let reason = if path.is_empty() {
                "Empty path".to_string()
            } else if !path.has_valid_length() {
                format!("Too long: {} chars", path.len())
            } else if !path.has_valid_characters() {
                "Invalid characters".to_string()
            } else {
                "Unknown validation error".to_string()
            };
            invalid.push((path, reason));
        }
    }
    
    (valid, invalid)
}
```

## Integration Examples

### With File I/O

```rust
use arrow_sus_shared::utils::path::{ValidatedPath, PathLike};
use std::fs;

fn read_config_file(path_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Validate path first
    let path = ValidatedPath::new(path_input)?;
    
    // Check if file exists
    if !path.as_path().exists() {
        return Err(format!("Config file not found: {}", path.to_string_lossy()).into());
    }
    
    // Ensure it's a file, not a directory
    if !path.as_path().is_file() {
        return Err(format!("Path is not a file: {}", path.to_string_lossy()).into());
    }
    
    // Read the file
    let content = fs::read_to_string(path.as_path())?;
    Ok(content)
}
```

### With Serde

```rust
use arrow_sus_shared::utils::path::ValidatedPath;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(with = "validated_path_serde")]
    output_dir: ValidatedPath,
    #[serde(with = "validated_path_serde")]
    input_file: ValidatedPath,
}

mod validated_path_serde {
    use super::*;
    use serde::{Deserializer, Serializer, Deserialize};
    
    pub fn serialize<S>(path: &ValidatedPath, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&path.to_string_lossy())
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<ValidatedPath, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValidatedPath::new(s).map_err(serde::de::Error::custom)
    }
}
```

### With CLI Applications

```rust
use arrow_sus_shared::utils::path::{ValidatedPath, PathValidatable};
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Input file path
    #[arg(short, long)]
    input: String,
    
    /// Output directory path
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Validate paths early
    let input_path = ValidatedPath::new(&args.input)
        .map_err(|e| format!("Invalid input path '{}': {}", args.input, e))?;
    
    let output_path = ValidatedPath::new(&args.output)
        .map_err(|e| format!("Invalid output path '{}': {}", args.output, e))?;
    
    // Ensure input exists and is a file
    if !input_path.as_path().exists() {
        return Err(format!("Input file does not exist: {}", input_path.to_string_lossy()).into());
    }
    
    if !input_path.as_path().is_file() {
        return Err(format!("Input path is not a file: {}", input_path.to_string_lossy()).into());
    }
    
    // Ensure output directory exists or can be created
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    
    // Process files...
    process_files(input_path, output_path)?;
    
    Ok(())
}
```

## Testing

The path utilities include comprehensive test coverage through doctests:

```bash
# Run all path-related doctests
cargo test --doc path

# Run specific doctest
cargo test --doc "PathValidatable"
```

All public APIs include working examples that serve as both documentation and tests, ensuring the code examples remain accurate and functional.

## Migration Guide

### From std::path

```rust
// Old approach
use std::path::{Path, PathBuf};

let path_str = "/some/path";
let path = Path::new(path_str);
if path.exists() {
    // Process path
}

// New approach
use arrow_sus_shared::utils::path::{ValidatedPath, PathLike};

let path_str = "/some/path";
let path = ValidatedPath::new(path_str)?; // Validation happens here
if path.as_path().exists() {
    // Process validated path
}
```

### From String-based Paths

```rust
// Old approach
fn process_path(path: String) -> Result<(), Box<dyn std::error::Error>> {
    if path.is_empty() {
        return Err("Empty path".into());
    }
    // Manual validation...
    let pathbuf = PathBuf::from(path);
    // Process pathbuf
    Ok(())
}

// New approach
use arrow_sus_shared::utils::path::ValidatedPath;

fn process_path(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let validated_path = ValidatedPath::new(path)?; // All validation handled
    // Process validated path
    Ok(())
}
```

The ArrowSUS path utilities provide a robust, performant, and type-safe foundation for all path-related operations in your applications. By leveraging comprehensive validation, detailed error handling, and optimized data structures, you can build reliable file system interactions with confidence.
