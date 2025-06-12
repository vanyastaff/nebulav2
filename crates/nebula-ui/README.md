# nebula-ui

[![Crates.io](https://img.shields.io/crates/v/nebula-ui.svg)](https://crates.io/crates/nebula-ui)
[![Documentation](https://docs.rs/nebula-ui/badge.svg)](https://docs.rs/nebula-ui)
[![License](https://img.shields.io/crates/l/nebula-ui.svg)](LICENSE)

Visual workflow editor for Nebula workflow engine, built with egui.

## Overview

A desktop application for creating and managing workflows visually:

- **Node Graph Editor** - Drag-and-drop workflow design
- **Live Preview** - See workflow execution in real-time
- **Node Library** - Browse and search available actions
- **Property Editor** - Configure node parameters
- **Debug Tools** - Step through executions and inspect data

## Screenshots

![Workflow Editor](docs/images/editor.png)
*Main workflow editor interface*

![Node Properties](docs/images/properties.png)
*Node configuration panel*

## Installation

### From Binary

Download the latest release for your platform from the [releases page](https://github.com/your-org/nebula/releases).

### From Source

```bash
# Clone repository
git clone https://github.com/your-org/nebula.git
cd nebula

# Build and run
cargo run --release --bin nebula-ui
```

### Package Managers

```bash
# macOS (Homebrew)
brew install nebula-ui

# Windows (Scoop)
scoop install nebula-ui

# Linux (Snap)
snap install nebula-ui
```

## Usage

### Connecting to Server

On first launch, configure the server connection:

1. Click **Settings** → **Server Connection**
2. Enter server URL (e.g., `http://localhost:8080`)
3. Enter credentials if authentication is enabled
4. Click **Test Connection**

### Creating a Workflow

1. Click **File** → **New Workflow** or press `Ctrl+N`
2. Drag nodes from the library panel to the canvas
3. Connect nodes by dragging from output to input ports
4. Configure node properties in the right panel
5. Save with `Ctrl+S`

### Node Operations

- **Add Node**: Drag from library or right-click canvas
- **Delete Node**: Select and press `Delete`
- **Duplicate Node**: `Ctrl+D`
- **Copy/Paste**: `Ctrl+C` / `Ctrl+V`
- **Select Multiple**: Hold `Shift` and click
- **Pan Canvas**: Middle mouse or `Space` + drag
- **Zoom**: Mouse wheel or `Ctrl` + `+`/`-`

### Testing Workflows

1. Click **Run** button or press `F5`
2. Provide test input if required
3. Watch execution progress in real-time
4. View results in the output panel

### Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| New Workflow | `Ctrl+N` |
| Open | `Ctrl+O` |
| Save | `Ctrl+S` |
| Save As | `Ctrl+Shift+S` |
| Run | `F5` |
| Stop | `Shift+F5` |
| Delete Node | `Delete` |
| Duplicate | `Ctrl+D` |
| Undo | `Ctrl+Z` |
| Redo | `Ctrl+Y` |
| Zoom In | `Ctrl++` |
| Zoom Out | `Ctrl+-` |
| Zoom Fit | `Ctrl+0` |

## Features

### Visual Editor

- **Node Graph**: Interactive canvas with pan and zoom
- **Connection Validation**: Prevents invalid connections
- **Auto-Layout**: Automatic graph layout algorithms
- **Minimap**: Overview navigation for large workflows
- **Grid Snapping**: Align nodes to grid

### Node Library

- **Categories**: Browse nodes by category
- **Search**: Find nodes by name or description
- **Favorites**: Pin frequently used nodes
- **Documentation**: Built-in node documentation

### Execution & Debug

- **Live Execution**: Watch data flow through nodes
- **Breakpoints**: Pause execution at specific nodes
- **Data Inspector**: Examine node inputs/outputs
- **Execution History**: Review past executions
- **Error Highlighting**: Visual error indicators

### Productivity

- **Templates**: Start from pre-built workflows
- **Snippets**: Save and reuse node groups
- **Export/Import**: Share workflows as JSON
- **Version Control**: Built-in Git integration
- **Multi-Window**: Work on multiple workflows

## Configuration

### Settings File

Settings are stored in:
- **Windows**: `%APPDATA%\nebula\settings.toml`
- **macOS**: `~/Library/Application Support/nebula/settings.toml`
- **Linux**: `~/.config/nebula/settings.toml`

```toml
[connection]
server_url = "http://localhost:8080"
auth_token = "your-token"
timeout_seconds = 30

[editor]
theme = "dark"  # or "light"
grid_size = 20
auto_save = true
auto_save_interval_seconds = 300

[appearance]
font_size = 14
show_minimap = true
highlight_connections = true

[shortcuts]
# Custom key bindings
run_workflow = "F5"
save_workflow = "Ctrl+S"
```

### Themes

The UI supports custom themes:

```toml
# ~/.config/nebula/themes/my-theme.toml
[colors]
background = "#1e1e1e"
panel = "#252526"
text = "#cccccc"
accent = "#007acc"
error = "#f44747"
success = "#4ec9b0"

[nodes]
default_color = "#3c3c3c"
selected_color = "#094771"
```

## Development

### Building from Source

```bash
# Debug build
cargo build --bin nebula-ui

# Release build with optimizations
cargo build --release --bin nebula-ui

# Run with logging
RUST_LOG=nebula_ui=debug cargo run --bin nebula-ui
```

### Architecture

```
nebula-ui/
├── src/
│   ├── main.rs          # Application entry point
│   ├── app.rs           # Main application state
│   ├── editor/          # Workflow editor
│   │   ├── canvas.rs    # Node graph canvas
│   │   ├── node.rs      # Node rendering
│   │   └── connection.rs # Connection handling
│   ├── panels/          # UI panels
│   │   ├── library.rs   # Node library
│   │   ├── properties.rs # Property editor
│   │   └── output.rs    # Execution output
│   ├── api/             # Server communication
│   └── utils/           # Helper functions
```

## Troubleshooting

### Connection Issues

- Verify server is running and accessible
- Check firewall settings
- Try using IP address instead of hostname
- Check server logs for errors

### Performance

- Disable minimap for large workflows
- Reduce grid size in settings
- Close unused panels
- Update graphics drivers

### Rendering Issues

- Try different renderer: `WGPU_BACKEND=gl cargo run`
- Disable vsync in settings
- Check GPU compatibility

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for development guidelines.

## Features

- `tracing` - Enable debug logging

## System Requirements

- **OS**: Windows 10+, macOS 10.15+, Linux (X11/Wayland)
- **RAM**: 4GB minimum, 8GB recommended
- **GPU**: OpenGL 3.3+ or Vulkan support
- **Resolution**: 1280x720 minimum

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.