# Matrix Terminal Dock Integration Guide

This guide explains how to get Matrix Terminal working properly with the macOS Dock.

## Method 1: Using the Create App Script (Recommended)

1. Open Terminal and navigate to the Matrix Terminal directory:
   ```
   cd ~/Documents/Projects/Terminal/Matrix
   ```

2. Run the updated app creation script:
   ```
   ./create_app.sh
   ```

   This script will:
   - Build Matrix Terminal in release mode
   - Create a proper macOS application bundle
   - Install the app in your Applications folder
   - Add the app to your Dock
   - Set up file associations

3. Once completed, you should see the Matrix Terminal icon in your Dock. Click it to launch.

## Method 2: Using the Direct Terminal App

1. Open Terminal and navigate to the Matrix Terminal directory:
   ```
   cd ~/Documents/Projects/Terminal/Matrix
   ```

2. Run the terminal app script:
   ```
   ./terminal_app.sh
   ```

3. The script will create a MatrixDirect.app in both the project folder and your Applications folder.

4. Drag MatrixDirect.app from Finder to your Dock to add it.

## Method 3: Manual Installation

If neither method works, you can manually install:

1. Build Matrix Terminal in release mode:
   ```
   cd ~/Documents/Projects/Terminal/Matrix
   cargo build --release
   ```

2. Create a simple AppleScript application:
   - Open Script Editor (Applications > Utilities > Script Editor)
   - Paste the following script:
     ```applescript
     on run
         tell application "Terminal"
             do script "cd ~/Documents/Projects/Terminal/Matrix && ./target/release/Matrix"
             set custom title of front window to "Matrix Terminal"
             activate
         end tell
     end run
     ```
   - Save as Application named "Matrix Terminal" to your Applications folder
   - In the Save dialog, check "Stay open after run handler"

3. Add a custom icon:
   - In Finder, locate MatrixTerminal.app
   - Right-click > Get Info
   - Drag AppIcon.icns from ~/Documents/Projects/Terminal/Matrix/MatrixTerminal.app/Contents/Resources/ to the icon in the top-left of the Info panel

4. Add to Dock:
   - Drag the saved application from Applications to your Dock

## Troubleshooting

If the Dock icon doesn't work properly:

1. **Permissions issues**: Make sure all scripts are executable:
   ```
   chmod +x ~/Documents/Projects/Terminal/Matrix/create_app.sh
   chmod +x ~/Documents/Projects/Terminal/Matrix/terminal_app.sh
   chmod +x ~/Documents/Projects/Terminal/Matrix/MatrixTerminal.app/Contents/MacOS/MatrixLauncher
   ```

2. **App bundle issues**: Try removing and reinstalling:
   ```
   rm -rf ~/Applications/MatrixTerminal.app
   ./create_app.sh
   ```

3. **Dock issues**: Reset the Dock:
   ```
   killall Dock
   ```

4. **Missing icons**: Make sure the app has an icon file:
   ```
   ls -la ~/Documents/Projects/Terminal/Matrix/MatrixTerminal.app/Contents/Resources/
   ```
   
   You should see AppIcon.icns in the list.

## Using the Matrix Terminal

Once successfully set up:

1. Click the Matrix Terminal icon in your Dock
2. The application will launch inside Terminal with proper dock integration
3. You can use Cmd+Tab to switch to Matrix Terminal
4. The Matrix-style sidebar will be available on the left with minimal green icons
5. Hover over the icons to see functionality tooltips