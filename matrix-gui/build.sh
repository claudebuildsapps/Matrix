#!/bin/bash
# Build script for Matrix Terminal GUI version

set -e  # Exit on error

# Navigate to project root
cd "$(dirname "$0")"

# Check that Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Error: Rust is not installed"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Parse arguments
RELEASE=false
RUN=false

for arg in "$@"; do
    case $arg in
        --release)
            RELEASE=true
            ;;
        --run)
            RUN=true
            ;;
        --help)
            echo "Usage: ./build.sh [--release] [--run]"
            echo ""
            echo "Options:"
            echo "  --release    Build in release mode"
            echo "  --run        Run the application after building"
            echo "  --help       Show this help message"
            exit 0
            ;;
    esac
done

# Build command
BUILD_CMD="cargo build"
if [ "$RELEASE" = true ]; then
    BUILD_CMD="$BUILD_CMD --release"
    BIN_PATH="./target/release/matrix_terminal_gui"
else
    BIN_PATH="./target/debug/matrix_terminal_gui"
fi

# Run the build
echo "Building Matrix Terminal GUI..."
echo "$BUILD_CMD"
$BUILD_CMD

# Show success message
echo "Build successful!"

# Run if requested
if [ "$RUN" = true ]; then
    echo "Running Matrix Terminal GUI..."
    $BIN_PATH
fi