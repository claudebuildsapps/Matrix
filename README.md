# Matrix Terminal

A Matrix-inspired terminal multiplexer built as a native GUI application.

## Overview

Matrix Terminal is a powerful terminal multiplexer designed for developers who need advanced organization, customization, and navigation features. This GUI version provides all the functionality of a traditional terminal multiplexer (like tmux or screen) in a standalone application with a distinctive Matrix aesthetic.

## Key Features

- **Native GUI Application**: Built with Rust and the iced GUI framework
- **Terminal Multiplexing**: Manage multiple terminal sessions in one window
- **Flexible Layout System**: Split terminals horizontally or vertically with nested layouts
- **Matrix-Inspired Design**: Green-on-black color scheme with non-rounded borders
- **Sidebar Navigation**: Quick access to commands via minimal Matrix-style icons
- **Layout Presets**: Quickly arrange terminals in common patterns (grid, horizontal, vertical, main+stack)
- **Window Zooming**: Focus on a single terminal temporarily
- **Keyboard Navigation**: Navigate between terminals using keyboard shortcuts

## Architecture

The application is built around several core components:

1. **Terminal Emulation**: Using alacritty_terminal for high-performance terminal emulation
2. **Process Management**: Using portable-pty for cross-platform pseudo-terminal handling
3. **GUI Framework**: Using iced for a native, lightweight user interface
4. **Layout Engine**: Custom layout management system for organizing terminals

See [ARCHITECTURE_GUI.md](ARCHITECTURE_GUI.md) for detailed information about the application architecture.

## Building

### Prerequisites

- Rust (1.70.0 or newer)
- Cargo
- Development libraries (X11/Wayland on Linux, nothing special on macOS/Windows)

### Build Commands

```bash
# Build and run the simple GUI prototype (recommended for initial testing)
./build_gui.sh --simple --run

# Build and run the full application
./build_gui.sh --run

# Build in debug mode (default is release)
./build_gui.sh --debug

# Get help on build options
./build_gui.sh --help
```

### Dock Integration (macOS)

To create a dock icon that auto-updates when you recompile:

```bash
# Create a dock-ready app bundle that auto-updates from source
./matrix_dock_icon.sh

# Then drag MatrixTerminal.app to your Applications folder or Dock
```

When you recompile your code with `./build_gui.sh`, the app will automatically use the new version next time you launch it from the dock.

### Developer Workflow

For a smoother development experience, use the auto-rebuild script:

```bash
# Watch the simple GUI for changes and rebuild automatically
./watch_and_build.sh

# Watch the full app for changes and rebuild automatically
./watch_and_build.sh --full

# Watch and build in release mode
./watch_and_build.sh --release
```

This requires `fswatch`, which can be installed via Homebrew: `brew install fswatch`.

You can also build directly with Cargo:

```bash
# Build the simple prototype
cd simple-gui
cargo build --release

# Build the full application
cd matrix-gui
cargo build --release
```

## Usage

### Keyboard Shortcuts

- **Ctrl+N**: Create a new terminal
- **Ctrl+H**: Split current terminal horizontally
- **Ctrl+V**: Split current terminal vertically
- **Ctrl+W**: Close current terminal
- **Ctrl+Tab**: Cycle through terminals
- **Ctrl+Arrow Keys**: Navigate between terminals by direction
- **Ctrl+Z**: Toggle zoom on current terminal
- **Ctrl+G**: Arrange terminals in a grid
- **Ctrl+Shift+H**: Arrange terminals horizontally
- **Ctrl+Shift+V**: Arrange terminals vertically
- **Ctrl+M**: Arrange terminals with current one as main
- **Ctrl+B**: Toggle sidebar

### Sidebar

The sidebar provides quick access to all major functions:

- **N**: Create new terminal
- **H**: Split horizontally
- **V**: Split vertically
- **G**: Grid layout
- **=**: Horizontal layout
- **‖**: Vertical layout
- **M**: Main layout
- **Z**: Toggle zoom
- **X**: Close window
- **?**: Help

## Development Status

This is a work in progress. Current status:

- [x] Architecture design
- [x] Basic framework setup
- [x] Matrix styling implementation (colors, borders, theme)
- [x] Simple GUI prototype
- [x] Sidebar with hover effects
- [x] Mock terminal interface
- [x] Layout system design
- [ ] Full terminal emulation integration
- [ ] Process management
- [ ] Complete layout management implementation
- [ ] Session persistence
- [ ] Advanced features

### Prototypes

1. **Simple GUI Prototype**: A demonstration of the Matrix-style UI with sidebar and mock terminal
   - Run with: `./build_gui.sh --simple --run`
   - Features interactive sidebar with hover effects
   - Shows basic Matrix styling and non-rounded borders
   - Provides a mock terminal with basic keyboard input

2. **Full GUI Implementation**: The complete application (in progress)
   - Run with: `./build_gui.sh --run`
   - Currently implementing terminal emulation integration

## License

[MIT License](LICENSE)

## Acknowledgments

- Inspired by the Matrix movie aesthetic
- Built with the excellent [iced](https://github.com/iced-rs/iced) GUI framework
- Terminal emulation provided by [alacritty_terminal](https://github.com/alacritty/alacritty)
