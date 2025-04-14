#!/bin/bash
# Fix Matrix Terminal dock icon

echo "Matrix Terminal Dock Icon Fixer"
echo "==============================="
echo

MATRIX_PATH="/Users/joshkornreich/Documents/Projects/Terminal/Matrix"
cd "$MATRIX_PATH"

echo "Step 1: Building Matrix Terminal in release mode..."
cargo build --release
echo

echo "Step 2: Fixing app bundle permissions..."
chmod +x "$MATRIX_PATH/MatrixTerminal.app/Contents/MacOS/MatrixLauncher"
echo

echo "Step 3: Updating app bundle Info.plist..."
cat > "$MATRIX_PATH/MatrixTerminal.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>MatrixLauncher</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>com.matrixterminal.app</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Matrix Terminal</string>
    <key>CFBundleDisplayName</key>
    <string>Matrix Terminal</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>10.11</string>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2025 Josh Kornreich. All rights reserved.</string>
    <key>NSUIElement</key>
    <false/>
    <key>NSAppleEventsUsageDescription</key>
    <string>Matrix Terminal needs to control Terminal to run properly.</string>
</dict>
</plist>
EOF
echo

echo "Step 4: Installing Matrix Terminal to Applications folder..."
APP_USER_DIR="$HOME/Applications/MatrixTerminal.app"
rm -rf "$APP_USER_DIR"
mkdir -p "$HOME/Applications"
cp -R "$MATRIX_PATH/MatrixTerminal.app" "$APP_USER_DIR"
chmod +x "$APP_USER_DIR/Contents/MacOS/MatrixLauncher"
echo

echo "Step 5: Adding to Dock..."
# First, check if the app is already in the Dock
if ! defaults read com.apple.dock persistent-apps | grep -q "MatrixTerminal.app"; then
    echo "Adding MatrixTerminal.app to Dock..."
    defaults write com.apple.dock persistent-apps -array-add "<dict><key>tile-data</key><dict><key>file-data</key><dict><key>_CFURLString</key><string>$APP_USER_DIR</string><key>_CFURLStringType</key><integer>0</integer></dict></dict></dict>"
    killall Dock
else
    echo "MatrixTerminal is already in Dock, removing and re-adding..."
    # Save current dock items to a temporary file
    defaults write com.apple.dock persistent-apps > /tmp/dock_apps.plist
    # Filter out MatrixTerminal
    grep -v "MatrixTerminal.app" /tmp/dock_apps.plist > /tmp/dock_apps_filtered.plist
    # Load filtered dock apps
    defaults write com.apple.dock persistent-apps -array $(cat /tmp/dock_apps_filtered.plist)
    # Add MatrixTerminal back
    defaults write com.apple.dock persistent-apps -array-add "<dict><key>tile-data</key><dict><key>file-data</key><dict><key>_CFURLString</key><string>$APP_USER_DIR</string><key>_CFURLStringType</key><integer>0</integer></dict></dict></dict>"
    # Restart the Dock
    killall Dock
    # Clean up
    rm -f /tmp/dock_apps.plist /tmp/dock_apps_filtered.plist
fi
echo

echo "Step 6: Creating alternative app for direct launching..."
"$MATRIX_PATH/terminal_app.sh"
echo

echo "Done! You can now use Matrix Terminal from the dock. If you still have issues,"
echo "please see the detailed instructions in DOCK_SETUP.md"