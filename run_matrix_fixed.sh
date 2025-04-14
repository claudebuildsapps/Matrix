#!/bin/bash
# Enhanced Matrix Terminal Launcher

MATRIX_PATH="/Users/joshkornreich/Documents/Projects/Terminal/Matrix"
cd "$MATRIX_PATH"

# Check if the terminal supports standard input
if [ -t 0 ]; then
    HAS_STDIN=true
else
    HAS_STDIN=false
fi

# Parse arguments
DEBUG_MODE=false
for arg in "$@"; do
    if [ "$arg" == "--debug" ]; then
        DEBUG_MODE=true
    fi
done

# Build if needed
if [ "$DEBUG_MODE" == "true" ]; then
    echo "Building Matrix Terminal in debug mode..."
    cargo build
    
    # Build the debug helper
    echo "Building debug helper..."
    cargo build --manifest-path Cargo-debug.toml
    
    echo "Starting Matrix Terminal in debug mode..."
    RUST_BACKTRACE=1 RUST_LOG=debug ./target/debug/Matrix
else
    echo "Building Matrix Terminal..."
    cargo build --release
    
    # Terminal detection logic
    if [ "$HAS_STDIN" == "true" ]; then
        # We're in an interactive terminal
        echo "Starting Matrix Terminal..."
        RUST_BACKTRACE=1 ./target/release/Matrix
    else
        # We're not in an interactive terminal - likely launched from Dock
        # Launch in a new Terminal window
        osascript <<EOT
        tell application "Terminal"
            do script "cd '$MATRIX_PATH' && TERM=xterm-256color RUST_BACKTRACE=1 ./target/release/Matrix; exit"
            set custom title of front window to "Matrix Terminal"
            activate
        end tell
EOT
    fi
fi

# If we get here, check if Matrix closed unexpectedly
if [ $? -ne 0 ] && [ "$DEBUG_MODE" == "false" ]; then
    echo "Matrix Terminal exited unexpectedly. Running in debug mode..."
    # Ask user if they want to try debug mode
    read -p "Would you like to try running in debug mode? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        $0 --debug
    fi
fi