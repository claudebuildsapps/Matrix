#!/bin/bash
# This script creates a simple Mac app launcher that will work reliably

# Set paths
MATRIX_PATH="/Users/joshkornreich/Documents/Projects/Terminal/Matrix"
OUTPUT_PATH="$MATRIX_PATH/SimpleMatrixLauncher.app"

# Create app structure
mkdir -p "$OUTPUT_PATH/Contents/MacOS"
mkdir -p "$OUTPUT_PATH/Contents/Resources"

# Create Info.plist
cat > "$OUTPUT_PATH/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>MatrixLauncher</string>
    <key>CFBundleIdentifier</key>
    <string>com.matrix.terminal</string>
    <key>CFBundleName</key>
    <string>Matrix Terminal</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

# Create launcher script
cat > "$OUTPUT_PATH/Contents/MacOS/MatrixLauncher" << EOF
#!/bin/bash
# Ultra simple launcher that just opens Terminal with Matrix

# Open Terminal with our script
open -a Terminal "$MATRIX_PATH/run_direct.command"
EOF

# Create direct run command file
cat > "$MATRIX_PATH/run_direct.command" << EOF
#!/bin/bash
# Direct Matrix launcher
cd "$MATRIX_PATH"
RUST_BACKTRACE=1 ./target/release/Matrix
EOF

# Copy icon if available
if [ -f "$MATRIX_PATH/MatrixTerminal.app/Contents/Resources/AppIcon.icns" ]; then
    cp "$MATRIX_PATH/MatrixTerminal.app/Contents/Resources/AppIcon.icns" "$OUTPUT_PATH/Contents/Resources/"
fi

# Make files executable
chmod +x "$OUTPUT_PATH/Contents/MacOS/MatrixLauncher"
chmod +x "$MATRIX_PATH/run_direct.command"

echo "============================="
echo "Simple Matrix Launcher created"
echo "============================="
echo "App is located at: $OUTPUT_PATH"
echo "To use it:"
echo "1. Open Finder to this location"
echo "2. Drag SimpleMatrixLauncher.app to your Dock"
echo "3. Click the icon in your Dock to launch"
echo
echo "You can also double-click run_direct.command to launch directly."