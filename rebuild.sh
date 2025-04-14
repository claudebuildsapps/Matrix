#!/bin/bash

# Simple rebuild script for Matrix Terminal
set -e

# Directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Display Matrix-style ASCII art for fun
echo -e "\033[32m"
echo "╔═══════════════════════════════════════════════════════╗"
echo "║                 REBUILDING THE MATRIX                 ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo -e "\033[0m"

# Build the simple-gui prototype in release mode and run it
echo "Building application..."
./build_gui.sh --simple --release

echo "Running application..."
./matrix-gui/target/release/matrix_simple_gui

echo "Done!"