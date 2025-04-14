#!/bin/bash
# Advanced debugging for Matrix Terminal

MATRIX_PATH="/Users/joshkornreich/Documents/Projects/Terminal/Matrix"
DEBUG_LOG="$MATRIX_PATH/matrix_debug.log"

# Start with a fresh log
echo "=== Matrix Terminal Debug Log $(date) ===" > "$DEBUG_LOG"
echo "Working directory: $MATRIX_PATH" >> "$DEBUG_LOG"

# Check if we're in a terminal
if [ -t 0 ]; then
  echo "Running in interactive terminal" >> "$DEBUG_LOG"
else
  echo "Not running in interactive terminal" >> "$DEBUG_LOG"
fi

# Check if necessary files exist
echo -e "\nChecking files:" >> "$DEBUG_LOG"
for file in run_matrix.sh run_matrix_fixed.sh launch_terminal.sh MatrixTerminal.app/Contents/MacOS/MatrixLauncher target/release/Matrix target/debug/Matrix; do
  if [ -f "$MATRIX_PATH/$file" ]; then
    echo "✓ $file exists ($(ls -la "$MATRIX_PATH/$file" | awk '{print $1,$3,$4}'))" >> "$DEBUG_LOG"
  else
    echo "✗ $file does not exist" >> "$DEBUG_LOG"
  fi
done

# Check permissions
echo -e "\nChecking executable permissions:" >> "$DEBUG_LOG"
for file in run_matrix.sh run_matrix_fixed.sh launch_terminal.sh MatrixTerminal.app/Contents/MacOS/MatrixLauncher; do
  if [ -x "$MATRIX_PATH/$file" ]; then
    echo "✓ $file is executable" >> "$DEBUG_LOG"
  else
    echo "✗ $file is not executable" >> "$DEBUG_LOG"
    # Fix permission
    chmod +x "$MATRIX_PATH/$file"
    echo "  Permission fixed" >> "$DEBUG_LOG"
  fi
done

# Try to build the program in both debug and release modes
echo -e "\nBuilding Matrix Terminal:" >> "$DEBUG_LOG"
cd "$MATRIX_PATH"

echo "Building debug version..." >> "$DEBUG_LOG"
cargo build 2>> "$DEBUG_LOG"
if [ $? -eq 0 ]; then
  echo "✓ Debug build successful" >> "$DEBUG_LOG"
else
  echo "✗ Debug build failed" >> "$DEBUG_LOG"
fi

echo "Building release version..." >> "$DEBUG_LOG"
cargo build --release 2>> "$DEBUG_LOG"
if [ $? -eq 0 ]; then
  echo "✓ Release build successful" >> "$DEBUG_LOG"
else
  echo "✗ Release build failed" >> "$DEBUG_LOG"
fi

# Check system environment
echo -e "\nSystem information:" >> "$DEBUG_LOG"
echo "Operating system: $(uname -a)" >> "$DEBUG_LOG"
echo "Terminal: $TERM" >> "$DEBUG_LOG"
echo "Shell: $SHELL" >> "$DEBUG_LOG"
echo "User: $(whoami)" >> "$DEBUG_LOG"
echo "Path: $PATH" >> "$DEBUG_LOG"

# Launch in debug mode with maximum verbosity
echo -e "\nStarting Matrix Terminal in debug mode..." >> "$DEBUG_LOG"

# Run this in a new Terminal.app window
osascript <<EOT >> "$DEBUG_LOG" 2>&1
tell application "Terminal"
  do script "cd '$MATRIX_PATH' && RUST_BACKTRACE=1 RUST_LOG=trace ./target/debug/Matrix 2> $MATRIX_PATH/matrix_error.log"
  set custom title of front window to "Matrix Terminal - DEBUG MODE"
  activate
end tell
EOT

echo -e "\nDebug startup attempted. Check matrix_error.log for errors." >> "$DEBUG_LOG"
echo "Debug log created at: $DEBUG_LOG"
echo "Launch in Terminal manually with: cd $MATRIX_PATH && RUST_LOG=trace ./target/debug/Matrix"