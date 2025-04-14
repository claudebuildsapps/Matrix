#!/bin/bash

# Create a macOS app bundle for the Matrix Terminal GUI
set -e

# Directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# First build the application
echo -e "${GREEN}Building Matrix Terminal GUI...${NC}"
./build_gui.sh --simple

# Create the app bundle structure
APP_NAME="MatrixTerminalGUI.app"
echo -e "Creating app bundle: ${GREEN}$APP_NAME${NC}"

# Remove the existing app bundle if it exists
if [ -d "$APP_NAME" ]; then
    echo "Removing existing app bundle..."
    rm -rf "$APP_NAME"
fi

# Create the directory structure
mkdir -p "$APP_NAME/Contents/MacOS"
mkdir -p "$APP_NAME/Contents/Resources"

# Create Info.plist
cat > "$APP_NAME/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>MatrixGUILauncher</string>
    <key>CFBundleIconFile</key>
    <string>matrix_icon</string>
    <key>CFBundleIdentifier</key>
    <string>com.matrix.terminal.gui</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Matrix Terminal GUI</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSUIElement</key>
    <false/>
</dict>
</plist>
EOF

# Create launcher script
cat > "$APP_NAME/Contents/MacOS/MatrixGUILauncher" << EOF
#!/bin/bash

# Get the directory where this script is located
SCRIPT_DIR="\$( cd "\$( dirname "\${BASH_SOURCE[0]}" )" && pwd )"
APP_DIR="\$(dirname "\$(dirname "\$SCRIPT_DIR")")"

# Run the executable
"\$SCRIPT_DIR/matrix_simple_gui"
EOF

# Make the launcher script executable
chmod +x "$APP_NAME/Contents/MacOS/MatrixGUILauncher"

# Copy the icon if it exists
if [ -f "matrix_icon.icns" ]; then
    cp "matrix_icon.icns" "$APP_NAME/Contents/Resources/matrix_icon.icns"
else
    echo -e "${RED}Warning: matrix_icon.icns not found. Using default icon.${NC}"
    # Create a simple icon if needed
fi

# Copy the executable
cp "matrix-gui/target/release/matrix_simple_gui" "$APP_NAME/Contents/MacOS/"

echo -e "${GREEN}App bundle created successfully!${NC}"
echo -e "You can now open ${GREEN}$APP_NAME${NC} by double-clicking on it."
echo -e "To install it, drag it to your Applications folder."