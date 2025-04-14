#!/bin/bash

# Simple launcher script for Matrix Terminal

# First build the project
echo "Building Matrix Terminal..."
cargo build || { echo "Build failed!"; exit 1; }

# Then run it
echo "Launching Matrix Terminal..."
./target/debug/Matrix

# If there was an error running the terminal directly, 
# try the preview script as a fallback
if [ $? -ne 0 ]; then
    echo "Could not launch Matrix Terminal directly."
    echo "Launching preview visualization instead..."
    python3 matrix_preview.py
fi