# Matrix Terminal - GUI Architecture

## Overview

Matrix Terminal is being redesigned as a native GUI application while maintaining its distinctive Matrix-inspired aesthetic. This document outlines the architectural approach for transitioning from a terminal-based UI (TUI) to a graphical UI (GUI) while preserving the core functionality.

## Technology Stack

- **Primary Language**: Rust
- **GUI Framework**: [iced](https://github.com/iced-rs/iced)
  - Chosen for its native Rust implementation and good performance characteristics
  - Provides cross-platform compatibility
  - Has a reactive, elm-like architecture that works well with our state management needs
- **Terminal Emulation**: [alacritty_terminal](https://github.com/alacritty/alacritty/tree/master/alacritty_terminal)
  - High-performance terminal emulation library written in Rust
  - Provides VT100/xterm compatibility
  - Handles terminal state, cursor positioning, and character attributes
- **Process Management**: [portable-pty](https://github.com/wez/wezterm/tree/main/pty)
  - Cross-platform pseudoterminal implementation
  - Used for spawning and managing shell processes

## Core Components

### 1. Application Shell

The application shell provides the main window and UI framework:

```rust
struct MatrixApp {
    // Application state
    windows: HashMap<Uuid, TerminalWindow>,
    layout_manager: LayoutManager,
    sidebar: Sidebar,
    
    // Configuration
    settings: Settings,
    
    // UI state
    current_view: ViewMode,
    modal_state: Option<ModalType>,
}
```

### 2. Terminal Window Component

Each terminal window encapsulates a terminal session:

```rust
struct TerminalWindow {
    id: Uuid,
    title: String,
    term: alacritty_terminal::Term,
    pty: Box<dyn PtyHandle>,
    config: TermConfig,
    
    // Visual state
    size: Size,
    position: Position,
    is_focused: bool,
    
    // Buffer and history
    scrollback: ScrollbackBuffer,
}
```

### 3. Layout Management

Maintains the same hierarchical layout system from the TUI version but adapted for GUI rendering:

```rust
enum LayoutNode {
    Window(Uuid),
    Split {
        direction: SplitDirection,
        ratio: f32,
        first: Box<LayoutNode>,
        second: Box<LayoutNode>,
    },
}

struct LayoutManager {
    root: Option<LayoutNode>,
    focused_window: Option<Uuid>,
    zoomed_window: Option<Uuid>,
    pre_zoom_layout: Option<LayoutNode>,
}
```

### 4. Matrix-Style Sidebar

A sidebar component that provides access to core functionality:

```rust
struct Sidebar {
    visible: bool,
    hovered_icon: Option<SidebarIcon>,
    clicked_icon: Option<SidebarIcon>,
    icons: Vec<SidebarIconDef>,
    width: f32,
}

enum SidebarIcon {
    NewWindow,
    SplitHorizontal,
    SplitVertical,
    // ...etc
}
```

## Visual Style

- Matrix green color scheme (primary: RGB 0, 255, 65)
- Non-rounded, square borders for terminals and UI elements
- Minimalist interface with green-on-black color scheme
- Hover tooltips for interface elements
- Custom Matrix-inspired font rendering (optional future enhancement)

## Implementation Phases

### Phase 1: Basic Framework (Current)
- Set up iced GUI framework integration
- Create basic window with terminal rendering
- Implement process spawning and terminal interaction

### Phase 2: Core Terminal Features
- Multiple terminal window support
- Basic window management
- Terminal configuration options

### Phase 3: Layout System
- Implement layout tree structure
- Split view functionality
- Window focus management

### Phase 4: Matrix UI Elements
- Sidebar implementation
- Matrix-style theming
- Non-rounded border customization

### Phase 5: Advanced Features
- Layout presets
- Window zooming
- Session management
- Configuration persistence

## Event Flow

1. User interactions (keyboard/mouse) are captured by the iced framework
2. Events are dispatched to appropriate components (terminal windows, sidebar, etc.)
3. Terminal input is forwarded to the corresponding PTY
4. PTY output updates the terminal state via alacritty_terminal
5. UI is re-rendered to reflect state changes

## Cross-Platform Considerations

While the initial focus is on macOS, the architecture will maintain cross-platform compatibility:

- Abstract platform-specific code into dedicated modules
- Use cross-platform libraries for core functionality
- Implement platform-specific optimizations where necessary

## Migration Path

The transition from TUI to GUI will be gradual, maintaining backward compatibility where possible:

1. Core logic (layout management, window tracking) will be ported first
2. UI components will be reimplemented using the GUI framework
3. Terminal-specific code will be adapted to use alacritty_terminal
4. Settings and configuration will be normalized between versions