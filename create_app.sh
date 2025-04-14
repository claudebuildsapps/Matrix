#!/bin/bash
# Create a native AppleScript application for Matrix Terminal

# Set the project path - update this if the project moves
MATRIX_PATH="$HOME/Documents/Projects/Terminal/Matrix"

# Build the Matrix application if needed
echo "Building Matrix application..."
cd "$MATRIX_PATH"
cargo build --release

# Create the application bundle structure
APP_NAME="MatrixTerminal"
APP_DIR="$MATRIX_PATH/$APP_NAME.app"  # Create in project directory
APP_USER_DIR="$HOME/Applications/$APP_NAME.app"  # User Applications folder path
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

echo "Creating application bundle in $APP_DIR"
rm -rf "$APP_DIR"  # Start fresh
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Create the .app contents
cat > "$RESOURCES_DIR/launcher.applescript" << EOT
-- MatrixTerminal.app Launcher
on run
    try
        -- Path to Matrix binary
        set matrixPath to "$MATRIX_PATH"
        set matrixBin to matrixPath & "/target/release/Matrix"
        
        -- Create the file system entity
        set matrixBinFile to POSIX file matrixBin as string
        
        tell application "Terminal"
            activate
            -- Launch a new terminal with Matrix
            do script "cd '" & matrixPath & "' && ./target/release/Matrix"
            
            -- Rename window/tab for better identification
            set custom title of front window to "Matrix Terminal"
        end tell
        
        -- Keep this app running to show in Command+Tab
        set isRunning to true
        
        -- Keep the AppleScript app alive while Matrix is running
        repeat while isRunning
            delay 2
            
            -- Check if Matrix is still running
            try
                tell application "Terminal"
                    if not (exists window 1) then
                        set isRunning to false
                    end if
                end tell
            on error
                set isRunning to false
            end try
        end repeat
        
    on error errMsg
        display dialog "Error launching Matrix Terminal: " & errMsg buttons {"OK"} default button "OK" with icon stop
    end try
end run
EOT

# Compile the AppleScript into a proper app
echo "Compiling AppleScript application..."
osacompile -o "$APP_DIR" "$RESOURCES_DIR/launcher.applescript"

# Create a proper Info.plist
cat > "$CONTENTS_DIR/Info.plist" << EOT
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>applet</string>
    <key>CFBundleIconFile</key>
    <string>applet</string>
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
    <key>CFBundleSignature</key>
    <string>aplt</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.14</string>
    <key>LSUIElement</key>
    <false/>
    <key>NSAppleEventsUsageDescription</key>
    <string>Matrix Terminal needs to control Terminal to run properly.</string>
    <key>NSSystemAdministrationUsageDescription</key>
    <string>Matrix Terminal uses AppleScript to launch Terminal with your preferences.</string>
</dict>
</plist>
EOT

# Process the icon
echo "Processing icons..."

# Copy the SVG icon
cp "$MATRIX_PATH/matrix_icon.svg" "$RESOURCES_DIR/matrix_icon.svg"

# Convert SVG to PNG for icon generation
if command -v convert &> /dev/null; then
    # Use ImageMagick if available
    convert -background none "$MATRIX_PATH/matrix_icon.svg" -resize 1024x1024 "$RESOURCES_DIR/matrix_icon.png"
    echo "Icon converted to PNG using ImageMagick"
elif command -v rsvg-convert &> /dev/null; then
    # Use rsvg-convert if available
    rsvg-convert -h 1024 -w 1024 -o "$RESOURCES_DIR/matrix_icon.png" "$MATRIX_PATH/matrix_icon.svg"
    echo "Icon converted to PNG using rsvg-convert"
else
    echo "Warning: Could not convert SVG to PNG, using SVG directly"
fi

# Create icns file for macOS
if command -v sips &> /dev/null && command -v iconutil &> /dev/null && [ -f "$RESOURCES_DIR/matrix_icon.png" ]; then
    echo "Creating icns icon file..."
    
    # Create iconset directory
    ICONSET_DIR="$RESOURCES_DIR/applet.iconset"
    mkdir -p "$ICONSET_DIR"
    
    # Generate various icon sizes
    sips -z 16 16 "$RESOURCES_DIR/matrix_icon.png" --out "$ICONSET_DIR/icon_16x16.png"
    sips -z 32 32 "$RESOURCES_DIR/matrix_icon.png" --out "$ICONSET_DIR/icon_16x16@2x.png"
    sips -z 32 32 "$RESOURCES_DIR/matrix_icon.png" --out "$ICONSET_DIR/icon_32x32.png"
    sips -z 64 64 "$RESOURCES_DIR/matrix_icon.png" --out "$ICONSET_DIR/icon_32x32@2x.png"
    sips -z 128 128 "$RESOURCES_DIR/matrix_icon.png" --out "$ICONSET_DIR/icon_128x128.png"
    sips -z 256 256 "$RESOURCES_DIR/matrix_icon.png" --out "$ICONSET_DIR/icon_128x128@2x.png"
    sips -z 256 256 "$RESOURCES_DIR/matrix_icon.png" --out "$ICONSET_DIR/icon_256x256.png"
    sips -z 512 512 "$RESOURCES_DIR/matrix_icon.png" --out "$ICONSET_DIR/icon_256x256@2x.png"
    sips -z 512 512 "$RESOURCES_DIR/matrix_icon.png" --out "$ICONSET_DIR/icon_512x512.png"
    sips -z 1024 1024 "$RESOURCES_DIR/matrix_icon.png" --out "$ICONSET_DIR/icon_512x512@2x.png"
    
    # Convert the iconset to icns
    iconutil -c icns "$ICONSET_DIR" -o "$RESOURCES_DIR/applet.icns"
    
    # Clean up
    rm -rf "$ICONSET_DIR"
    echo "Created icns icon file"
fi

# Touch the app to refresh Finder/Dock
touch "$APP_DIR"

# Copy application to user's Applications folder
echo "Copying application to Applications folder..."
cp -R "$APP_DIR" "$APP_USER_DIR"

# Make sure the app is executable
chmod +x "$APP_DIR/Contents/MacOS/applet"
chmod +x "$APP_USER_DIR/Contents/MacOS/applet"

# Add the app to the dock if it's not already there
if ! defaults read com.apple.dock persistent-apps | grep -q "$APP_NAME.app"; then
    echo "Adding app to Dock..."
    defaults write com.apple.dock persistent-apps -array-add "<dict><key>tile-data</key><dict><key>file-data</key><dict><key>_CFURLString</key><string>$APP_USER_DIR</string><key>_CFURLStringType</key><integer>0</integer></dict></dict></dict>"
    killall Dock
fi

# Set the application as the default for .matrix files
# First create a UTI (Uniform Type Identifier) for .matrix files
defaults write com.apple.LaunchServices/com.apple.launchservices.secure LSHandlers -array-add '{LSHandlerContentType=public.data;LSHandlerRoleAll=com.matrixterminal.app;}'

# Reset LaunchServices to apply changes
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -kill -r -domain local -domain system -domain user

echo "Application bundle created at $APP_DIR"
echo "Application also copied to $APP_USER_DIR"
echo "You can now launch Matrix Terminal from the Dock and use Command+Tab to switch to it"