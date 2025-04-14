#!/bin/bash
# The simplest possible Matrix Terminal launcher
# This file can be double-clicked in Finder

# Go to the Matrix directory
cd "$(dirname "$0")"

# Build Matrix Terminal (release mode)
echo "Building Matrix Terminal..."
cargo build --release

# Set proper terminal environment variables
export TERM=xterm-256color
export RUST_BACKTRACE=1

# Launch Matrix Terminal
echo "Starting Matrix Terminal..."
./target/release/Matrix

# If the app exited with an error, keep the window open
if [ $? -ne 0 ]; then
    echo "Matrix Terminal encountered an error."
    echo "Press Enter to close this window."
    read
fi