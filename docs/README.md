# ArrowSUS Documentation

This directory contains the complete documentation setup for ArrowSUS, combining both mdBook narrative documentation and Rust API documentation.

## 📁 Structure

```
docs/
├── README.md           # This file
├── book.toml          # mdBook configuration
├── src/               # mdBook source files
│   ├── SUMMARY.md     # Table of contents
│   ├── introduction.md
│   ├── getting-started/
│   ├── core/
│   ├── utils/
│   ├── examples/
│   └── ...
└── theme/             # Custom styling
    ├── custom.css     # Custom CSS
    └── custom.js      # Custom JavaScript
```

## 🚀 Quick Start

### Build All Documentation

```bash
# Using the build script
./scripts/build-docs.sh

# Using just (if installed)
just docs
```

### Build and Serve Locally

```bash
# Using the build script
./scripts/build-docs.sh --serve

# Using just (if installed)
just docs-serve
```

### Build Individual Components

```bash
# mdBook only
cd docs && mdbook build

# Cargo docs only
cd rust && cargo doc --all-features --no-deps
```

## 📖 Documentation Types

### 1. mdBook Documentation
- **Location**: `docs/src/`
- **Output**: `docs/book/` → `target/documentation/`
- **Purpose**: Narrative documentation, tutorials, guides
- **Features**:
  - Custom CSS styling with Catppuccin Mocha theme and ArrowSUS branding
  - Copy buttons for code blocks
  - External link indicators
  - Smooth scrolling
  - Search functionality

### 2. Cargo API Documentation
- **Location**: `rust/crates/as_shared/src/`
- **Output**: `rust/target/doc/` → `target/documentation/api/`
- **Purpose**: API reference, doctests, examples
- **Features**:
  - All public and private items documented
  - Doctests for examples
  - Cross-references between modules

## 🛠️ Available Commands

### Using the Build Script

```bash
# Build complete documentation
./scripts/build-docs.sh

# Build and serve on http://localhost:8000
./scripts/build-docs.sh --serve

# Clean previous builds
./scripts/build-docs.sh --clean

# Show help
./scripts/build-docs.sh --help
```

### Using Just (Recommended)

```bash
# Show all available commands
just

# Build complete documentation
just docs

# Build and serve documentation
just docs-serve

# Clean documentation
just docs-clean

# Build only mdBook
just mdbook

# Build only cargo docs
just cargo-docs

# Start mdBook development server
just mdbook-serve

# Watch for changes and rebuild
just watch

# Show documentation statistics
just stats
```

## 📊 Current Statistics

After running `just docs`, you'll see statistics like:

```
📊 Documentation Statistics:
  📖 mdBook pages: 36
  🦀 Cargo doc files: 44
  💾 Total size: 5.4M
  📁 Output directory: target/documentation
```

## 🎨 Customization

### Custom CSS
Edit `theme/custom.css` to modify:
- Color scheme (Catppuccin Mocha theme with ArrowSUS branding)
- Typography
- Layout adjustments
- Dark/light theme variations

### Custom JavaScript
Edit `theme/custom.js` to add:
- Interactive features
- Copy buttons
- External link handling
- Smooth scrolling

### mdBook Configuration
Edit `book.toml` to configure:
- Book metadata
- Output settings
- Preprocessors
- Theme options

## 📝 Writing Documentation

### Adding New Pages

1. Create a new `.md` file in the appropriate `src/` subdirectory
2. Add it to `src/SUMMARY.md` in the correct location
3. Use the established structure and style

### Documentation Standards

- **Headers**: Use descriptive, hierarchical headers
- **Code Examples**: Include working, tested examples
- **Links**: Use relative links for internal references
- **Images**: Store in `src/images/` if needed
- **Admonitions**: Use custom CSS classes for notes, warnings, tips

### Example Page Structure

```markdown
# Page Title

Brief introduction to the topic.

## Overview

Explain what this covers.

## Basic Usage

```rust
use arrow_sus_shared::module::Type;

let example = Type::new()?;
```

## Advanced Topics

More complex examples and use cases.

## See Also

- [Related Topic](./related.md)
- [API Reference](../api/module.html)
```

## 🔧 Maintenance

### Updating Dependencies

```bash
# Update mdBook
cargo install mdbook --force

# Update Rust toolchain
rustup update
```

### Fixing Build Issues

1. **mdBook errors**: Check `book.toml` syntax
2. **Cargo doc errors**: Ensure all doctests compile
3. **Missing files**: Check `SUMMARY.md` links
4. **CSS/JS issues**: Validate custom theme files

### Performance Optimization

- Keep images optimized and reasonably sized
- Minimize custom CSS/JS
- Use mdBook's built-in features when possible
- Consider splitting very large pages

## 🚀 Deployment

The documentation is designed to be deployed as static files:

1. **Build**: `./scripts/build-docs.sh`
2. **Deploy**: Upload `target/documentation/` to your web server
3. **Serve**: Any static file server can host the documentation

### GitHub Pages Example

```yaml
# .github/workflows/docs.yml
name: Deploy Documentation

on:
  push:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Install mdBook
      run: cargo install mdbook
    - name: Build Documentation
      run: ./scripts/build-docs.sh
    - name: Deploy to GitHub Pages
      uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: ./target/documentation
```

## 🤝 Contributing

When contributing to documentation:

1. Follow the existing structure and style
2. Test your changes locally before submitting
3. Update `SUMMARY.md` for new pages
4. Ensure all links work correctly
5. Run the full build to check for errors

## 📚 Resources

- [mdBook Documentation](https://rust-lang.github.io/mdBook/)
- [Rust Doc Book](https://doc.rust-lang.org/rustdoc/)
- [Just Command Runner](https://github.com/casey/just)
- [Arrow-SUS Repository](https://github.com/wrath-codes/arrow-sus)