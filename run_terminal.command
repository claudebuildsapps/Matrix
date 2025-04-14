#!/bin/bash
# Simple direct launcher for Matrix Terminal

# Change to the Matrix Terminal directory
cd "$(dirname "$0")"

# Make sure the terminal is built
if [ ! -f "target/release/Matrix" ]; then
  echo "Building Matrix Terminal..."
  cargo build --release
fi

# Set proper terminal environment
export TERM=xterm-256color

# Launch the application
echo "Launching Matrix Terminal..."
./target/release/Matrix

# Keep window open if there was an error
if [ $? -ne 0 ]; then
  echo "Matrix Terminal exited with an error."
  echo "Press Enter to close this window..."
  read
fi