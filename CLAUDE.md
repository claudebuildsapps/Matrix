# Matrix Terminal Development Guide

## Version Number
- Increment version number by 0.1 with each significant update
- Current version: v0.6

## Build Instructions
- Use `./build_gui.sh --simple` to build the simple prototype

## Common Tasks
- Update version number in `simple-gui/src/main.rs` (in the `VERSION` constant)
- Test keyboard navigation using F1-F4 keys or Ctrl+Arrow keys to switch between terminal panes
- Test window resizing with Ctrl+Shift+Arrow keys
- Test scrolling behavior for continuous input and command execution

## Keyboard Shortcuts
- **Terminal Navigation**
  - Arrow keys: Navigate between split terminals (no modifier needed)
  - F1-F4: Focus specific terminal panes (alternative method)
- **Window Resizing**
  - Ctrl+Shift+↑: Decrease window height
  - Ctrl+Shift+↓: Increase window height
  - Ctrl+Shift+←: Decrease window width
  - Ctrl+Shift+→: Increase window width

## UI Improvements
- Secondary terminals use compact headers to save space
- Terminal prompts show "neo@matrix:~$" in line with Matrix theme
- Cursor position is tracked across all terminals when navigating between them
- Cursor is only shown in the currently focused terminal
- All terminals support basic command input handling independently
- Primary terminal has full command set, secondary terminals have a reduced command set (clear, whoami, echo)