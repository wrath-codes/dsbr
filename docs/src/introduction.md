# ArrowSUS Documentation

Welcome to the **ArrowSUS** documentation! ArrowSUS is a comprehensive Rust library that provides foundations for data processing, including domain models, schemas, parsing utilities, IO traits, and optional integrations with Polars and FTP tooling.

## What is ArrowSUS?

ArrowSUS (Arrow Shared Utilities and Structures) is designed to be the foundational layer for data processing applications. It provides:

- **Domain Models**: Well-structured data types for common business entities
- **Parsing Utilities**: Robust parsers for various data formats
- **IO Abstractions**: Flexible traits for different data sources (FTP, S3, local filesystem)
- **Time Utilities**: Comprehensive date and time manipulation tools
- **Error Handling**: Consistent error types and result patterns
- **Optional Integrations**: Seamless integration with Polars and Apache Arrow

## Key Features

### 🏗️ **Solid Architecture**
Built with clean architecture principles, separating concerns between core functionality, domain logic, and utilities.

### 🔧 **Flexible IO**
Support for multiple data sources including FTP servers, AWS S3, and local file systems with a unified interface.

### ⚡ **Performance Focused**
Leverages Rust's zero-cost abstractions and optional Polars integration for high-performance data processing.

### 🛡️ **Type Safety**
Comprehensive error handling and type-safe operations throughout the library.

### 📊 **Data Format Support**
Built-in support for various data formats including DBF/DBC files, with extensible parsing capabilities.

## Quick Example

```rust
use arrow_sus_shared::utils::time::month::Month;
use arrow_sus_shared::core::Result;

fn main() -> Result<()> {
    // Create a month from different inputs
    let jan = Month::from_number(1)?;
    let feb = Month::from_english_name("February")?;
    let mar = Month::from_abbreviation("Mar")?;
    
    // Navigate between months
    let next_month = jan.next();
    let prev_month = feb.previous();
    
    // Calculate differences
    let months_between = jan.months_until(&mar);
    
    println!("January to March: {} months", months_between);
    
    Ok(())
}
```

## Getting Started

Ready to dive in? Check out our [Installation Guide](./getting-started/installation.md) to get ArrowSUS set up in your project, or jump straight to the [Quick Start](./getting-started/quick-start.md) for a hands-on introduction.

## Architecture Overview

ArrowSUS is organized into several key modules:

- **`core`**: Fundamental types, error handling, and result patterns
- **`domain`**: Business domain models and entities
- **`utils`**: Utility functions for time, paths, and metadata operations
- **`io`**: Input/output abstractions and implementations

For a deeper understanding of the architecture, see our [Architecture Overview](./core/architecture.md).

## Community and Support

- **Repository**: [GitHub](https://github.com/wrath-codes/arrowsus)
- **Documentation**: [docs.rs](https://docs.rs/arrow-sus-shared)
- **Issues**: [GitHub Issues](https://github.com/wrath-codes/arrowsus/issues)

## License

ArrowSUS is licensed under the [MIT License](./license.md).