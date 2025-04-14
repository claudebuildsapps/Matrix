# Matrix Terminal Sidebar

The Matrix Terminal sidebar is a minimalist control panel that provides quick access to the terminal multiplexer's functionality. It's designed with a Matrix-style green aesthetic, using sharp corners and a digital look.

## Sidebar Features

- **Minimal Design**: The sidebar is only 3 characters wide, showing only the icon characters.
- **Matrix Green Aesthetic**: Uses the iconic Matrix green color (#00FF41) for main elements.
- **Box-Style Borders**: Uses non-rounded square borders for a digital/retro look.
- **Hover Effects**: Icons highlight when hovered over.
- **Instant Tooltips**: Shows function description and keyboard shortcuts on hover.
- **Left-Aligned**: Attached to the left side of the terminal for easy access.

## Sidebar Icons and Functions

| Icon | Function | Description | Shortcut |
|------|----------|-------------|----------|
| `N` | New Window | Create a new terminal window | Ctrl+N or :new |
| `H` | Split Horizontal | Split window horizontally | Ctrl+H or :split h |
| `V` | Split Vertical | Split window vertically | Ctrl+V or :split |
| `G` | Grid Layout | Arrange windows in a grid | Ctrl+G or :layout grid |
| `=` | Horizontal Layout | Arrange windows horizontally | Ctrl+Shift+H or :layout h |
| `‖` | Vertical Layout | Arrange windows vertically | Ctrl+Shift+V or :layout v |
| `M` | Main Layout | Current window as main with others stacked | Ctrl+M or :layout main |
| `Z` | Zoom | Toggle zoom on current window | Ctrl+Z or :zoom |
| `X` | Close Window | Close the current window | Ctrl+W or :close |
| `?` | Help | Show help information | :help |

## How It Works

1. The sidebar is implemented as a component in `src/ui/sidebar.rs`.
2. Mouse events are captured to detect hover state over icons.
3. When an icon is hovered, a tooltip appears showing function and shortcut.
4. Clicking an icon executes the associated command.
5. The sidebar can be toggled on/off with Ctrl+B or the `:sidebar` command.

## Technical Implementation

- Uses ratatui library for rendering
- Mouse event handling for hover detection and click interaction
- Implements a Matrix-inspired color scheme using RGB colors
- BorderType::Plain for non-rounded box-style borders
- Integrated with window manager for executing commands
- Icon positions calculated based on terminal dimensions

The sidebar enhances the Matrix Terminal's usability by providing visual access to all multiplexer functions while maintaining a minimal, thematic interface.