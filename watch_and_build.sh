#!/bin/bash

# Watch for changes in source files and rebuild the project automatically
set -e

# Directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if fswatch is installed (needed for file watching)
if ! command -v fswatch &> /dev/null; then
    echo -e "${YELLOW}fswatch is required for this script to work.${NC}"
    echo -e "You can install it with Homebrew: ${BLUE}brew install fswatch${NC}"
    exit 1
fi

# Default settings
BUILD_TYPE="simple" # Can be "simple" or "full"
RELEASE_MODE=false

# Parse arguments
for arg in "$@"; do
    case $arg in
        --full)
            BUILD_TYPE="full"
            ;;
        --release)
            RELEASE_MODE=true
            ;;
        --help)
            echo "Usage: ./watch_and_build.sh [--full] [--release]"
            echo ""
            echo "Options:"
            echo "  --full       Watch and build the full application (default is simple)"
            echo "  --release    Build in release mode (default is debug)"
            echo "  --help       Show this help message"
            exit 0
            ;;
    esac
done

# Build command based on arguments
if [ "$BUILD_TYPE" = "simple" ]; then
    echo -e "${GREEN}Watching simple GUI project for changes...${NC}"
    SRC_DIR="simple-gui/src"
    BUILD_CMD="./build_gui.sh --simple"
else
    echo -e "${GREEN}Watching full GUI project for changes...${NC}"
    SRC_DIR="matrix-gui/src"
    BUILD_CMD="./build_gui.sh"
fi

if [ "$RELEASE_MODE" = true ]; then
    BUILD_CMD="$BUILD_CMD --release"
else
    BUILD_CMD="$BUILD_CMD --debug"
fi

# Initial build
echo -e "${BLUE}Performing initial build...${NC}"
$BUILD_CMD

# Function to handle file changes
handle_change() {
    echo -e "\n${YELLOW}File change detected!${NC}"
    echo -e "${BLUE}Building project...${NC}"
    $BUILD_CMD
    echo -e "${GREEN}Ready and watching for changes...${NC}"
}

# Watch for changes and rebuild
echo -e "${GREEN}Watching for changes. Press Ctrl+C to stop.${NC}"
fswatch -o "$SRC_DIR" | while read -r; do
    handle_change
done