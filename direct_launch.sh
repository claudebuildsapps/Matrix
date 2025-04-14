#!/bin/bash
# Ultra-simple Matrix Terminal launcher for troubleshooting

# Just open a terminal and run the program directly
osascript <<EOT
tell application "Terminal"
  do script "cd '/Users/joshkornreich/Documents/Projects/Terminal/Matrix' && ./target/release/Matrix"
  set custom title of front window to "Matrix Terminal"
  activate
end tell
EOT