#!/bin/bash
# This file can be double-clicked in Finder to launch Matrix Terminal 
# directly in a Terminal window.

# Move to the Matrix directory
cd "$(dirname "$0")"

# Build Matrix Terminal
echo "Building Matrix Terminal..."
cargo build --release

# Launch Matrix Terminal
echo "Starting Matrix Terminal..."
./target/release/Matrix

# Keep the window open if there was an error
if [ $? -ne 0 ]; then
    echo "Matrix Terminal exited with an error. Press Return to close this window."
    read
fi