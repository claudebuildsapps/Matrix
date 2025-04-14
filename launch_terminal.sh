#!/bin/bash
# Universal Matrix Terminal launcher that handles all platforms and launch methods

MATRIX_PATH=$(dirname "$0")
cd "$MATRIX_PATH"

# Function to check if we're in a TTY
in_terminal() {
  [ -t 0 ] && return 0 || return 1
}

# Function to launch in macOS Terminal
launch_in_terminal() {
  osascript <<EOT
  tell application "Terminal"
    do script "cd '$MATRIX_PATH' && TERM=xterm-256color RUST_BACKTRACE=1 ./target/release/Matrix"
    set custom title of front window to "Matrix Terminal"
    activate
  end tell
EOT
}

# Build the application if needed
echo "Building Matrix Terminal..."
cargo build --release

# Check if we're in a terminal
if in_terminal; then
  echo "Launching Matrix Terminal in current terminal..."
  RUST_BACKTRACE=1 ./target/release/Matrix
else
  echo "Launching Matrix Terminal in a new Terminal window..."
  launch_in_terminal
fi