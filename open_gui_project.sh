#!/bin/bash
# Script to open the GUI project in VS Code or other IDE

# Check if VS Code is installed
if command -v code &> /dev/null; then
    echo "Opening Matrix Terminal GUI project in VS Code..."
    code /Users/joshkornreich/Documents/Projects/Terminal/Matrix/matrix-gui
    exit 0
fi

# Check if Xcode or other IDEs are installed
if command -v xed &> /dev/null; then
    echo "Opening Matrix Terminal GUI project in Xcode..."
    xed /Users/joshkornreich/Documents/Projects/Terminal/Matrix/matrix-gui
    exit 0
fi

# If no IDE is found, open in Finder
echo "No IDE found. Opening in Finder..."
open /Users/joshkornreich/Documents/Projects/Terminal/Matrix/matrix-gui