#!/bin/bash

# Simple script to update the dock app with the latest binary
set -e

# Directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Build the application
echo -e "${GREEN}Building simple GUI application...${NC}"
./build_gui.sh --simple

# Copy the binary to the app bundle
echo -e "Updating dock application..."
SOURCE_BINARY="$SCRIPT_DIR/simple-gui/target/release/matrix_simple_gui"
TARGET_DIR="$SCRIPT_DIR/MatrixTerminal.app/Contents/MacOS/"

if [ -f "$SOURCE_BINARY" ]; then
    cp "$SOURCE_BINARY" "$TARGET_DIR"
    chmod +x "$TARGET_DIR/matrix_simple_gui"
    echo -e "${GREEN}Dock application updated successfully!${NC}"
    echo -e "You can now launch the app from your dock."
else
    echo -e "${RED}Error: Binary not found at $SOURCE_BINARY${NC}"
    echo -e "Please try building the application again."
    exit 1
fi