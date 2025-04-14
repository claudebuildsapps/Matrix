#!/bin/bash
# Create a more direct terminal app for macOS

MATRIX_PATH="/Users/joshkornreich/Documents/Projects/Terminal/Matrix"
APP_NAME="MatrixDirect"
APP_DIR="$MATRIX_PATH/$APP_NAME.app"

# Make sure the Matrix app is built
echo "Building Matrix in release mode..."
cd "$MATRIX_PATH" && cargo build --release

# Create the app structure
echo "Creating $APP_NAME.app..."
mkdir -p "$APP_DIR/Contents/"{MacOS,Resources}

# Create the Info.plist file
cat > "$APP_DIR/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleDevelopmentRegion</key>
	<string>English</string>
	<key>CFBundleExecutable</key>
	<string>MatrixDirect</string>
	<key>CFBundleIconFile</key>
	<string>matrix_icon</string>
	<key>CFBundleIdentifier</key>
	<string>com.matrixdirect.app</string>
	<key>CFBundleInfoDictionaryVersion</key>
	<string>6.0</string>
	<key>CFBundleName</key>
	<string>Matrix Direct</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>CFBundleShortVersionString</key>
	<string>1.0</string>
	<key>CFBundleSignature</key>
	<string>????</string>
	<key>CFBundleVersion</key>
	<string>1.0</string>
	<key>NSHighResolutionCapable</key>
	<true/>
	<key>NSHumanReadableCopyright</key>
	<string>Copyright © 2025 Josh Kornreich</string>
	<key>NSMainNibFile</key>
	<string>MainMenu</string>
	<key>NSPrincipalClass</key>
	<string>NSApplication</string>
</dict>
</plist>
EOF

# Create the launcher script
cat > "$APP_DIR/Contents/MacOS/MatrixDirect" << EOF
#!/bin/bash

# Get directory where script is located
SCRIPT_DIR="\$( cd "\$( dirname "\${BASH_SOURCE[0]}" )" && pwd )"
MATRIX_PATH="$MATRIX_PATH"
MATRIX_BIN="\$MATRIX_PATH/target/release/Matrix"

# Launch Terminal with the Matrix app
osascript <<EOT
tell application "Terminal"
    # Create a unique Terminal session with Matrix
    do script "cd '\$MATRIX_PATH' && TERM=xterm-256color exec '\$MATRIX_BIN'"
    
    # Set the title for better identification
    set custom title of front window to "Matrix Terminal"
    
    # Bring Terminal to front
    activate
end tell
EOT
EOF

# Make the script executable
chmod +x "$APP_DIR/Contents/MacOS/MatrixDirect"

# Copy the icon
cp -f "$MATRIX_PATH/MatrixTerminal.app/Contents/Resources/AppIcon.icns" "$APP_DIR/Contents/Resources/matrix_icon.icns"

# Install in Applications folder
APP_USER_DIR="$HOME/Applications/$APP_NAME.app"
rm -rf "$APP_USER_DIR"
cp -R "$APP_DIR" "$APP_USER_DIR"

echo "Created $APP_NAME.app at:"
echo "  - $APP_DIR"
echo "  - $APP_USER_DIR"
echo
echo "Now you can:"
echo "1. Add to Dock: drag the app from Finder to the Dock"
echo "2. Launch via Spotlight: press Cmd+Space and type 'Matrix Direct'"