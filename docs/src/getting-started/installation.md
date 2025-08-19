# Installation

This guide will help you get ArrowSUS set up in your Rust project.

## Prerequisites

- **Rust**: Version 1.70 or later (uses Rust 2024 edition)
- **Cargo**: Comes with Rust installation

## Adding to Your Project

### Basic Installation

Add ArrowSUS to your `Cargo.toml`:

```toml
[dependencies]
arrow-sus-shared = "0.1.0"
```

### With Optional Features

ArrowSUS provides several optional features for different use cases:

```toml
[dependencies]
arrow-sus-shared = { version = "0.1.0", features = ["polars", "ftp", "s3"] }
```

## Available Features

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `polars` | Enable Polars integration for high-performance data processing | `polars`, `rayon` |
| `ftp` | Enable FTP client capabilities | `suppaftp`, `tokio-rustls` |
| `s3` | Enable AWS S3 integration | `aws-config`, `aws-sdk-s3` |
| `geo` | Enable geospatial operations (future) | `geo`, `proj` |

### Feature Combinations

```toml
# Data processing focused
[dependencies]
arrow-sus-shared = { version = "0.1.0", features = ["polars"] }

# IO operations focused  
[dependencies]
arrow-sus-shared = { version = "0.1.0", features = ["ftp", "s3"] }

# Full feature set
[dependencies]
arrow-sus-shared = { version = "0.1.0", features = ["polars", "ftp", "s3"] }
```

## Development Installation

If you want to contribute to ArrowSUS or use the latest development version:

### Clone the Repository

```bash
git clone https://github.com/wrath-codes/arrowsus.git
cd arrowsus
```

### Build the Project

```bash
# Build with default features
cargo build

# Build with all features
cargo build --all-features

# Build specific features
cargo build --features "polars,ftp"
```

### Run Tests

```bash
# Run all tests
cargo test

# Run tests with all features
cargo test --all-features

# Run specific test modules
cargo test utils::time::month
```

## Verification

Create a simple test to verify your installation:

```rust
// src/main.rs or in a test file
use arrow_sus_shared::utils::time::month::Month;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let january = Month::from_number(1)?;
    println!("Month: {} ({})", january.name_en, january.name_ptbr);
    
    let next_month = january.next();
    println!("Next month: {}", next_month.name_en);
    
    Ok(())
}
```

Run it:

```bash
cargo run
```

Expected output:
```
Month: January (Janeiro)
Next month: February
```

## IDE Setup

### VS Code

For the best development experience with VS Code:

1. Install the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension
2. Install the [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) extension for debugging

### IntelliJ IDEA / CLion

1. Install the [Rust plugin](https://plugins.jetbrains.com/plugin/8182-rust)
2. Configure the toolchain in Settings → Languages & Frameworks → Rust

## Common Issues

### Compilation Errors

**Issue**: `error: package 'arrow-sus-shared' cannot be built because it requires rustc 1.70 or newer`

**Solution**: Update your Rust installation:
```bash
rustup update
```

**Issue**: Feature-related compilation errors

**Solution**: Make sure you're enabling the required features:
```toml
[dependencies]
arrow-sus-shared = { version = "0.1.0", features = ["required-feature"] }
```

### Dependency Conflicts

**Issue**: Version conflicts with other crates

**Solution**: Use `cargo tree` to identify conflicts and pin specific versions:
```bash
cargo tree --duplicates
```

### Performance Issues

**Issue**: Slow compilation times

**Solution**: Use the `--release` flag for optimized builds:
```bash
cargo build --release
```

Or configure your development profile in `Cargo.toml`:
```toml
[profile.dev]
opt-level = 1
```

## Next Steps

Once you have Arrow-SUS installed:

1. Check out the [Quick Start Guide](./quick-start.md) for a hands-on introduction
2. Review the [Configuration Guide](./configuration.md) for advanced setup options
3. Explore the [Examples](../examples/basic.md) for common usage patterns

## Getting Help

If you encounter issues during installation:

- Check the [GitHub Issues](https://github.com/wrath-codes/arrow-sus/issues) for known problems
- Create a new issue with your system details and error messages
- Join our community discussions for help from other users