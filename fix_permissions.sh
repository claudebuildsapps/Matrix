#!/bin/bash
# Fix permissions for all scripts

MATRIX_PATH="/Users/joshkornreich/Documents/Projects/Terminal/Matrix"
cd "$MATRIX_PATH"

echo "Fixing permissions for Matrix Terminal scripts..."

# Make all scripts executable
chmod +x run_matrix.sh
chmod +x run_matrix_fixed.sh
chmod +x launch_terminal.sh
chmod +x create_app.sh
chmod +x terminal_app.sh
chmod +x fix_dock_icon.sh
chmod +x fix_permissions.sh
chmod +x MatrixTerminal.app/Contents/MacOS/MatrixLauncher

# Copy to Applications if needed
APP_USER_DIR="$HOME/Applications/MatrixTerminal.app"
if [ -d "$APP_USER_DIR" ]; then
    echo "Updating MatrixLauncher in Applications folder..."
    cp -f "$MATRIX_PATH/MatrixTerminal.app/Contents/MacOS/MatrixLauncher" "$APP_USER_DIR/Contents/MacOS/"
    chmod +x "$APP_USER_DIR/Contents/MacOS/MatrixLauncher"
fi

# Touch files to update modification time
touch MatrixTerminal.app
touch "$APP_USER_DIR"

echo "Permissions fixed!"
echo "Try launching Matrix Terminal from the dock now."