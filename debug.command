#!/bin/bash
# Matrix Terminal Debug Launcher
# This file can be double-clicked in Finder to launch

cd "$(dirname "$0")"

# Create a log file
LOG_FILE="$(pwd)/debug_log.txt"
echo "=== Matrix Terminal Debug Log $(date) ===" > "$LOG_FILE"
echo "Working directory: $(pwd)" >> "$LOG_FILE"

# Build in debug mode
echo "Building Matrix Terminal in debug mode..." | tee -a "$LOG_FILE"
cargo build | tee -a "$LOG_FILE"

# Check if build succeeded
if [ ! -f "target/debug/Matrix" ]; then
    echo "Build failed! Matrix binary not found." | tee -a "$LOG_FILE"
    exit 1
fi

# Launch with debugging
echo "Launching Matrix Terminal with debugging..." | tee -a "$LOG_FILE"
RUST_BACKTRACE=full RUST_LOG=debug ./target/debug/Matrix 2>&1 | tee -a "$LOG_FILE"

echo "Process exited. Press Return to close this window."
read