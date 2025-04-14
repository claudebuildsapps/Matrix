#!/bin/bash

# Create a persistent macOS app bundle for Matrix Terminal GUI that auto-updates
set -e

# Directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# App configuration
APP_NAME="MatrixTerminal.app"
APP_ID="com.matrix.terminal.gui"
BINARY_PATH="matrix-gui/target/release/matrix_terminal_gui"
SIMPLE_BINARY_PATH="simple-gui/target/release/matrix_simple_gui"
USE_SIMPLE=true  # Set to false to use the full application instead of the simple version

echo -e "${GREEN}Creating Matrix Terminal dock app that auto-updates when recompiled${NC}"

# Create the app bundle structure
mkdir -p "$APP_NAME/Contents/MacOS"
mkdir -p "$APP_NAME/Contents/Resources"

# Create Info.plist
cat > "$APP_NAME/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>MatrixDockLauncher</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>${APP_ID}</string>
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
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSUIElement</key>
    <false/>
</dict>
</plist>
EOF

# Create a launcher script that checks for a new binary at runtime
# This is the key to auto-updating when the code is recompiled
cat > "$APP_NAME/Contents/MacOS/MatrixDockLauncher" << EOF
#!/bin/bash

# Get the directory where this script is located
SCRIPT_DIR="\$( cd "\$( dirname "\${BASH_SOURCE[0]}" )" && pwd )"
APP_ROOT="\$(dirname "\$(dirname "\$SCRIPT_DIR")")"
PROJECT_ROOT="\$APP_ROOT"  # The app is already in the project root

# Set USE_SIMPLE to same value as in the parent script
USE_SIMPLE=$USE_SIMPLE

# Determine which binary to use
if [ "\$USE_SIMPLE" = true ]; then
    BINARY_PATH="simple-gui/target/release/matrix_simple_gui"
    BINARY_NAME="matrix_simple_gui"
else
    BINARY_PATH="matrix-gui/target/release/matrix_terminal_gui"
    BINARY_NAME="matrix_terminal_gui"
fi

# Try to build the application using the script first
echo "Building application..."
# Only try this if the build script exists
if [ -f "\$PROJECT_ROOT/build_gui.sh" ]; then
    cd "\$PROJECT_ROOT" && ./build_gui.sh --simple || echo "Build script failed, will try direct cargo build if needed."
else
    echo "build_gui.sh not found, will try direct cargo build if needed."
fi

# Always use the fresh binary from the project
SOURCE_BINARY="\$PROJECT_ROOT/\$BINARY_PATH"

# Check if the source binary exists
if [ -f "\$SOURCE_BINARY" ]; then
    # Copy the binary to the app bundle if it's newer
    if [ ! -f "\$SCRIPT_DIR/\$BINARY_NAME" ] || [ "\$SOURCE_BINARY" -nt "\$SCRIPT_DIR/\$BINARY_NAME" ]; then
        echo "Updating application binary..."
        cp "\$SOURCE_BINARY" "\$SCRIPT_DIR/\$BINARY_NAME"
        chmod +x "\$SCRIPT_DIR/\$BINARY_NAME"
    fi
    
    # Run the application
    "\$SCRIPT_DIR/\$BINARY_NAME"
else
    # If the source binary doesn't exist, try building it directly
    echo "Source binary not found, building it automatically..."
    
    # Build directly using cargo instead of the script
    cd "\$PROJECT_ROOT/simple-gui" && cargo build --release
    
    # Check if that worked
    if [ -f "\$SOURCE_BINARY" ]; then
        # Copy the binary to the app bundle
        echo "Build successful, copying binary..."
        cp "\$SOURCE_BINARY" "\$SCRIPT_DIR/\$BINARY_NAME"
        chmod +x "\$SCRIPT_DIR/\$BINARY_NAME"
        
        # Run the application
        "\$SCRIPT_DIR/\$BINARY_NAME"
    else
        # Last resort error message
        BINARY_PATH_DISPLAY="\$PROJECT_ROOT/\$BINARY_PATH"
        osascript -e "display dialog \"Matrix Terminal binary still not found at $BINARY_PATH_DISPLAY after build attempt. Please check the project setup.\" buttons {\"OK\"} default button \"OK\" with icon stop with title \"Matrix Terminal Error\""
        exit 1
    fi
fi
EOF

# Make the launcher script executable
chmod +x "$APP_NAME/Contents/MacOS/MatrixDockLauncher"

# Copy icon if it exists
if [ -f "MatrixTerminal.app/Contents/Resources/AppIcon.icns" ]; then
    cp "MatrixTerminal.app/Contents/Resources/AppIcon.icns" "$APP_NAME/Contents/Resources/"
elif [ -f "icons/matrix_icon.icns" ]; then
    cp "icons/matrix_icon.icns" "$APP_NAME/Contents/Resources/AppIcon.icns"
else
    echo -e "${RED}Warning: Icon not found. Your app will use the default icon.${NC}"
fi

# Build the application initially
./build_gui.sh --simple

# Copy the binary
if [ "$USE_SIMPLE" = true ]; then
    SOURCE_BINARY="$SCRIPT_DIR/$SIMPLE_BINARY_PATH"
    BINARY_NAME=$(basename "$SIMPLE_BINARY_PATH")
else
    SOURCE_BINARY="$SCRIPT_DIR/$BINARY_PATH"
    BINARY_NAME=$(basename "$BINARY_PATH")
fi

if [ -f "$SOURCE_BINARY" ]; then
    cp "$SOURCE_BINARY" "$APP_NAME/Contents/MacOS/$BINARY_NAME"
    chmod +x "$APP_NAME/Contents/MacOS/$BINARY_NAME"
else
    echo -e "${RED}Error: Binary not found at $SOURCE_BINARY${NC}"
    echo -e "Please build the application first with ./build_gui.sh --simple"
    exit 1
fi

echo -e "${GREEN}App bundle created successfully!${NC}"
echo -e "To install, drag ${GREEN}$APP_NAME${NC} to your Applications folder."
echo -e "When you recompile the code, the app will automatically use the new version."

# If the user has the app in the dock already and it's in the Applications folder,
# we can update it directly (optional)
if [ -d "/Applications/$APP_NAME" ]; then
    read -p "Update in Applications folder? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo -e "Updating application in /Applications..."
        rm -rf "/Applications/$APP_NAME"
        cp -R "$APP_NAME" "/Applications/"
        echo -e "${GREEN}Updated successfully!${NC}"
    fi
fi