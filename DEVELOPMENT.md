# Development Guide for TerminalMatrix

This guide provides instructions for setting up and developing TerminalMatrix in Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (installed with Rust)
- Git

## Project Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/TerminalMatrix.git
   cd TerminalMatrix
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the project:
   ```bash
   cargo run
   ```

## Development Workflow

### Project Structure

```
src/
├── main.rs           # Application entry point
├── app.rs            # Main application logic
├── terminal/         # Terminal-related functionality
│   ├── mod.rs
│   ├── window.rs     # Terminal window implementation
│   └── emulation.rs  # Terminal emulation logic
├── ui/               # User interface components
│   ├── mod.rs
│   ├── colors.rs     # Color scheme implementation
│   └── layout.rs     # Layout management
├── config/           # Configuration handling
│   ├── mod.rs
│   └── settings.rs   # User settings
└── utils/            # Utility functions
    └── mod.rs
```

### Adding New Features

1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Implement the feature in the appropriate module
3. Write tests for your code
4. Ensure code passes all tests and lint checks:
   ```bash
   cargo test
   cargo clippy
   ```
5. Submit a pull request

### Testing

- Unit tests: `cargo test`
- Integration tests: `cargo test --test '*'`
- Single test: `cargo test test_name`

### Code Style

This project follows the Rust standard style guidelines:

- Run `cargo fmt` before committing to ensure consistent formatting
- Use `cargo clippy` to check for common mistakes and improve code quality

## Dependencies

Key dependencies include:

- `crossterm` - Terminal manipulation
- `tui` - Terminal user interface library
- `serde` - Serialization/deserialization for config
- `clap` - Command line argument parsing

## Building for Release

```bash
cargo build --release
```

The compiled binary will be available in `target/release/terminal_matrix`.

## Troubleshooting

If you encounter issues during development, please:

1. Check the GitHub issues to see if it's a known problem
2. Update to the latest dependencies
3. If the issue persists, create a new issue with detailed reproduction steps

## Documentation

Generate and view the documentation locally:

```bash
cargo doc --open
```