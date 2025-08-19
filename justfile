# ArrowSUS Documentation Justfile

# Default recipe - show help
default:
    @just --list

# Build complete documentation (mdBook + cargo docs)
docs:
    @echo "🚀 Building complete documentation..."
    ./scripts/build-docs.sh

# Build and serve documentation locally
docs-serve:
    @echo "🚀 Building and serving documentation..."
    ./scripts/build-docs.sh --serve

# Clean all documentation builds
docs-clean:
    @echo "🧹 Cleaning documentation..."
    ./scripts/build-docs.sh --clean

# Build only cargo documentation
cargo-docs:
    @echo "🦀 Building cargo documentation..."
    cargo doc --all-features --no-deps --document-private-items
    @echo "✅ Cargo docs built at target/doc/arrow_sus_shared/index.html"

# Build only mdBook documentation
mdbook:
    @echo "📖 Building mdBook documentation..."
    cd docs && mdbook build
    @echo "✅ mdBook built at docs/book/index.html"

# Test documentation builds
test-docs:
    @echo "🧪 Testing documentation builds..."
    cargo doc --all-features --no-deps
    cd docs && mdbook test
    @echo "✅ Documentation tests passed"

# Start mdBook development server
mdbook-serve:
    @echo "📖 Starting mdBook development server..."
    cd docs && mdbook serve --open

# Install documentation dependencies
install-deps:
    @echo "📦 Installing documentation dependencies..."
    cargo install mdbook
    @echo "✅ Dependencies installed"

# Check documentation links
check-links:
    @echo "🔗 Checking documentation links..."
    cd docs && mdbook test
    @echo "✅ Link check completed"

# Build cargo docs with specific features
cargo-docs-polars:
    @echo "🦀 Building cargo docs with Polars features..."
    cargo doc --features polars --no-deps

cargo-docs-ftp:
    @echo "🦀 Building cargo docs with FTP features..."
    cargo doc --features ftp --no-deps

cargo-docs-s3:
    @echo "🦀 Building cargo docs with S3 features..."
    cargo doc --features s3 --no-deps

# Open documentation in browser
open-docs:
    @echo "🌐 Opening documentation..."
    @if [ -f "target/documentation/index.html" ]; then \
        open target/documentation/index.html || xdg-open target/documentation/index.html; \
    else \
        echo "❌ Documentation not found. Run 'just docs' first."; \
    fi

# Watch and rebuild mdBook on changes
watch:
    @echo "👀 Watching for changes..."
    cd docs && mdbook watch

# Generate documentation statistics
stats:
    @echo "📊 Documentation Statistics:"
    @if [ -d "target/documentation" ]; then \
        echo "  📖 mdBook pages: $(find target/documentation -name "*.html" -not -path "*/api/*" | wc -l)"; \
        echo "  🦀 Cargo doc files: $(find target/documentation/api -name "*.html" 2>/dev/null | wc -l || echo 0)"; \
        echo "  💾 Total size: $(du -sh target/documentation | cut -f1)"; \
    else \
        echo "  ❌ No documentation found. Run 'just docs' first."; \
    fi