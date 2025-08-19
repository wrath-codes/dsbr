#!/bin/bash

# ArrowSUS Documentation Build Script
# This script builds both mdBook documentation and cargo docs

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DOCS_DIR="docs"
CARGO_DOC_DIR="rust/target/doc"
OUTPUT_DIR="target/documentation"
MDBOOK_OUTPUT="$DOCS_DIR/book"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command_exists cargo; then
        print_error "cargo is not installed. Please install Rust."
        exit 1
    fi
    
    if ! command_exists mdbook; then
        print_warning "mdbook is not installed. Installing..."
        cargo install mdbook
    fi
    
    print_success "Prerequisites check completed"
}

# Clean previous builds
clean_docs() {
    print_status "Cleaning previous documentation builds..."
    
    if [ -d "$OUTPUT_DIR" ]; then
        rm -rf "$OUTPUT_DIR"
    fi
    
    if [ -d "$MDBOOK_OUTPUT" ]; then
        rm -rf "$MDBOOK_OUTPUT"
    fi
    
    if [ -d "$CARGO_DOC_DIR" ]; then
        rm -rf "$CARGO_DOC_DIR"
    fi
    
    print_success "Cleanup completed"
}

# Build cargo documentation
build_cargo_docs() {
    print_status "Building cargo documentation..."
    
    cd rust
    
    # Build docs with all features
    cargo doc --all-features --no-deps --document-private-items
    
    cd ..
    
    print_success "Cargo documentation built successfully"
}

# Build mdBook documentation
build_mdbook() {
    print_status "Building mdBook documentation..."
    
    cd "$DOCS_DIR"
    
    # Build the book
    mdbook build
    
    cd ..
    
    print_success "mdBook documentation built successfully"
}

# Combine documentation
combine_docs() {
    print_status "Combining documentation outputs..."
    
    # Create output directory
    mkdir -p "$OUTPUT_DIR"
    
    # Copy mdBook output
    if [ -d "$MDBOOK_OUTPUT" ]; then
        cp -r "$MDBOOK_OUTPUT"/* "$OUTPUT_DIR/"
        print_success "mdBook content copied to $OUTPUT_DIR"
    else
        print_warning "mdBook output not found at $MDBOOK_OUTPUT"
    fi
    
    # Copy cargo docs
    if [ -d "$CARGO_DOC_DIR" ]; then
        mkdir -p "$OUTPUT_DIR/api"
        cp -r "$CARGO_DOC_DIR"/* "$OUTPUT_DIR/api/"
        print_success "Cargo docs copied to $OUTPUT_DIR/api"
    else
        print_warning "Cargo docs not found at $CARGO_DOC_DIR"
    fi
    
    # Create index redirect
    cat > "$OUTPUT_DIR/api/index.html" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Arrow-SUS API Documentation</title>
    <meta http-equiv="refresh" content="0; url=arrow_sus_shared/index.html">
    <link rel="canonical" href="arrow_sus_shared/index.html">
</head>
<body>
    <p>Redirecting to <a href="arrow_sus_shared/index.html">Arrow-SUS API Documentation</a>...</p>
</body>
</html>
EOF
    
    print_success "Documentation combination completed"
}

# Generate documentation statistics
generate_stats() {
    print_status "Generating documentation statistics..."
    
    local mdbook_pages=0
    local cargo_docs_files=0
    local total_size=0
    
    if [ -d "$OUTPUT_DIR" ]; then
        mdbook_pages=$(find "$OUTPUT_DIR" -name "*.html" -not -path "*/api/*" | wc -l)
        cargo_docs_files=$(find "$OUTPUT_DIR/api" -name "*.html" 2>/dev/null | wc -l || echo 0)
        total_size=$(du -sh "$OUTPUT_DIR" | cut -f1)
    fi
    
    echo ""
    echo "📊 Documentation Statistics:"
    echo "  📖 mdBook pages: $mdbook_pages"
    echo "  🦀 Cargo doc files: $cargo_docs_files"
    echo "  💾 Total size: $total_size"
    echo "  📁 Output directory: $OUTPUT_DIR"
    echo ""
}

# Serve documentation locally
serve_docs() {
    if [ "$1" = "--serve" ]; then
        print_status "Starting local documentation server..."
        
        if command_exists python3; then
            cd "$OUTPUT_DIR"
            print_success "Documentation server running at http://localhost:8000"
            print_status "Press Ctrl+C to stop the server"
            python3 -m http.server 8000
        elif command_exists python; then
            cd "$OUTPUT_DIR"
            print_success "Documentation server running at http://localhost:8000"
            print_status "Press Ctrl+C to stop the server"
            python -m SimpleHTTPServer 8000
        else
            print_warning "Python not found. Cannot start local server."
            print_status "You can manually open $OUTPUT_DIR/index.html in your browser"
        fi
    fi
}

# Main execution
main() {
    echo "🚀 Arrow-SUS Documentation Builder"
    echo "=================================="
    echo ""
    
    check_prerequisites
    clean_docs
    build_cargo_docs
    build_mdbook
    combine_docs
    generate_stats
    
    print_success "Documentation build completed successfully!"
    print_status "Documentation available at: $OUTPUT_DIR/index.html"
    
    serve_docs "$1"
}

# Handle command line arguments
case "${1:-}" in
    --help|-h)
        echo "Usage: $0 [OPTIONS]"
        echo ""
        echo "Options:"
        echo "  --serve    Build documentation and start local server"
        echo "  --clean    Clean documentation and exit"
        echo "  --help     Show this help message"
        echo ""
        echo "Examples:"
        echo "  $0                # Build documentation"
        echo "  $0 --serve        # Build and serve documentation"
        echo "  $0 --clean        # Clean previous builds"
        ;;
    --clean)
        clean_docs
        print_success "Documentation cleaned"
        ;;
    *)
        main "$1"
        ;;
esac