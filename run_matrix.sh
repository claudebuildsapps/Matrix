#!/bin/bash
# Script to run Matrix terminal

# Navigate to the project directory
cd "$(dirname "$0")"

# Check if debug mode is requested
if [[ "$1" == "--debug" ]]; then
    echo "Running Matrix Terminal in debug mode..."
    RUST_BACKTRACE=1 RUST_LOG=debug cargo run
else
    echo "Running Matrix Terminal..."
    # Check if we're in an interactive terminal
    if [ -t 0 ]; then
        # Direct launch
        cargo run --release
    else
        # Launched from dock or other non-interactive source
        osascript <<EOT
        tell application "Terminal"
            do script "cd '$(pwd)' && RUST_BACKTRACE=1 cargo run --release; exit"
            set custom title of front window to "Matrix Terminal"
            activate
        end tell
EOT
    fi
fi