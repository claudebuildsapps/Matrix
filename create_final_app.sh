#!/bin/bash
# Create a final, simpler app bundle for Matrix Terminal

MATRIX_PATH="/Users/joshkornreich/Documents/Projects/Terminal/Matrix"
APP_NAME="MatrixTerminal"
APP_PATH="$MATRIX_PATH/$APP_NAME.app"
DOCK_PATH="$HOME/Applications/$APP_NAME.app"

echo "Creating Matrix Terminal app bundle..."

# Create app directory structure
mkdir -p "$APP_PATH/Contents/MacOS"
mkdir -p "$APP_PATH/Contents/Resources"

# Create Info.plist
cat > "$APP_PATH/Contents/Info.plist" << EOT
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleExecutable</key>
	<string>MatrixLauncher</string>
	<key>CFBundleIconFile</key>
	<string>AppIcon</string>
	<key>CFBundleIdentifier</key>
	<string>com.joshkornreich.matrix-terminal</string>
	<key>CFBundleInfoDictionaryVersion</key>
	<string>6.0</string>
	<key>CFBundleName</key>
	<string>Matrix Terminal</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>CFBundleShortVersionString</key>
	<string>1.0</string>
	<key>CFBundleVersion</key>
	<string>1</string>
	<key>LSMinimumSystemVersion</key>
	<string>10.11</string>
	<key>NSHumanReadableCopyright</key>
	<string>Copyright © 2025 Josh Kornreich</string>
</dict>
</plist>
EOT

# Create launcher script
cat > "$APP_PATH/Contents/MacOS/MatrixLauncher" << EOT
#!/bin/bash
# Matrix Terminal Launcher

# Set the path to Matrix Terminal
MATRIX_PATH="$MATRIX_PATH"

# Open Terminal and run Matrix Terminal
open -a Terminal "\$MATRIX_PATH/launch_matrix.command"
EOT

# Copy icon if it exists
if [ -f "$MATRIX_PATH/MatrixTerminal.app/Contents/Resources/AppIcon.icns" ]; then
  cp "$MATRIX_PATH/MatrixTerminal.app/Contents/Resources/AppIcon.icns" "$APP_PATH/Contents/Resources/"
fi

# Make launcher script executable
chmod +x "$APP_PATH/Contents/MacOS/MatrixLauncher"

# Create a copy in ~/Applications
mkdir -p "$HOME/Applications"
rm -rf "$DOCK_PATH"
cp -R "$APP_PATH" "$DOCK_PATH"

echo "Matrix Terminal app bundle created at $APP_PATH"
echo "Copied to $DOCK_PATH for Dock use"
echo ""
echo "To use Matrix Terminal:"
echo "1. Double-click launch_matrix.command to run directly"
echo "2. Or, drag MatrixTerminal.app to the Dock and click the icon"
echo ""
echo "If the application still doesn't work properly, try manually running:"
echo "cd \"$MATRIX_PATH\" && RUST_BACKTRACE=1 ./target/release/Matrix"