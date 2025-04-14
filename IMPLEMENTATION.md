# Matrix Terminal Implementation Roadmap

This document outlines the implementation plan for Matrix Terminal, a Matrix-themed terminal multiplexer built as a native GUI application.

## 1. Terminal Emulation Integration

- [ ] **Basic Terminal Window**
  - [ ] Implement proper character rendering with monospace font
  - [ ] Add cursor drawing and blinking
  - [ ] Support for Matrix-green text on dark background
  - [ ] Implement proper terminal dimensions and resize handling

- [ ] **Process Spawning**
  - [ ] Use portable-pty to spawn a real shell process
  - [ ] Connect input/output between GUI and shell process
  - [ ] Implement proper environment variable handling
  - [ ] Support basic terminal escape sequences

## 2. Layout Management

- [ ] **Split Views**
  - [ ] Add support for horizontal splitting (side-by-side terminals)
  - [ ] Add support for vertical splitting (terminals stacked vertically)
  - [ ] Implement resize handles between split terminals
  - [ ] Support keyboard shortcuts for managing splits (Ctrl+H, Ctrl+V)

- [ ] **Window Navigation**
  - [ ] Add keyboard shortcuts to navigate between terminals
  - [ ] Implement focused window highlighting with Matrix green border
  - [ ] Add window zooming functionality (focus on one window temporarily)
  - [ ] Support for cycling through windows with Tab

## 3. Matrix-Style Enhancements

- [ ] **Terminal Effects**
  - [ ] Add subtle Matrix-style "raining code" effect in empty terminals
  - [ ] Implement prompt customization for shell (matrix-themed prompt)
  - [ ] Add Matrix-style animations for window transitions

- [ ] **UI Polish**
  - [ ] Enhance sidebar with better icon design and hover effects
  - [ ] Implement tooltips for all controls
  - [ ] Add a minimal Matrix-style status bar at the bottom
  - [ ] Create custom scrollbars with Matrix styling

## 4. Additional Features

- [ ] **Session Management**
  - [ ] Save and restore window layouts
  - [ ] Support for named sessions
  - [ ] Auto-save session state

- [ ] **Preferences**
  - [ ] Customizable keyboard shortcuts
  - [ ] Configurable Matrix theme settings
  - [ ] Font selection and sizing options

## Implementation Approach

1. First, integrate terminal emulation with alacritty_terminal library
2. Connect to real PTY for process execution using portable-pty
3. Implement window splitting and management 
4. Add Matrix-style effects and UI polish
5. Finally, add session management and preferences

## Current Status

- [x] GUI prototype with Matrix styling
- [x] Working dock integration
- [x] Sidebar component with hover effects
- [x] Mock terminal with keyboard input
- [ ] Real terminal emulation (in progress)