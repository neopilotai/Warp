# Warp Terminal UI Guide

## Overview

The **Warp Terminal UI** is a complete, production-ready terminal interface system written in pure Rust. It replicates the sophisticated design of the Warp terminal while providing a modular, extensible architecture for building advanced CLI applications.

## Architecture

### Core Components

#### 1. **Renderer** (`ui/renderer.rs`)
The low-level terminal rendering engine using `crossterm` for cross-platform compatibility.

```rust
let mut renderer = TerminalRenderer::new()?;
renderer.clear();
renderer.set_cursor(10, 5);
renderer.write("Hello, Terminal!");
renderer.flush()?;
```

**Features:**
- Raw terminal mode management
- ANSI color support (256 colors)
- Cursor positioning
- Alternative screen buffer
- Cross-platform (Windows, macOS, Linux)

#### 2. **Components** (`ui/components.rs`)
Reusable UI building blocks modeled after Warp's design:

- **Sidebar**: Hierarchical navigation with items, folders, workflows
- **TabBar**: Multi-tab interface for managing sessions
- **FileList**: Scrollable file/directory listing
- **CommandBar**: Command input with suggestions
- **ContentPanel**: Scrollable content display

```rust
let mut sidebar = Sidebar::new("Warp Drive".to_string());
sidebar.add_item("My Workflow".to_string(), SidebarItemType::Workflow, 0);
sidebar.select_next();  // Navigate with vim-style keys
```

#### 3. **Layout System** (`ui/layout.rs`)
Flexible layout engine for terminal UI positioning:

```rust
let layout = Layout::new(width, height);

// Split into sidebar + main content
let (sidebar_rect, content_rect) = layout.split_horizontal(25);

// Split content into header + main + footer
let (header, main, footer) = layout.split_vertical(2, 3);
```

**Layout capabilities:**
- Horizontal split (sidebar + content)
- Vertical split (header + main + footer)
- Flexible rectangle positioning
- Border drawing utilities

#### 4. **Style System** (`ui/styles.rs`)
Theme and color management:

```rust
let colors = ColorScheme::warp();  // Dark theme inspired by Warp
// Or: ColorScheme::dark(), ColorScheme::light()

let styled = colors.apply_style("Hello", &colors.primary);
println!("{}", styled);
```

**Color Schemes:**
- **Warp Theme**: Cyan accents on dark background
- **Dark Theme**: Alternative dark palette
- **Light Theme**: Light background with dark text

#### 5. **State Management** (`ui/state.rs`)
Unified UI state tracking and input handling:

```rust
let mut state = UIState::new();
state.focus_pane(FocusedPane::FileList);
state.handle_input('j');  // Vim-style navigation
```

**Focused Panes:**
- Sidebar
- TabBar
- FileList
- CommandBar

## Usage Examples

### Basic Terminal UI Application

```rust
use warp_terminal_apps::WarpTerminalUI;

fn main() -> std::io::Result<()> {
    let mut ui = WarpTerminalUI::new()?;
    ui.initialize_demo();
    ui.render()?;
    
    Ok(())
}
```

### Building Custom Components

```rust
use warp_terminal_apps::ui::*;

fn main() -> std::io::Result<()> {
    let mut renderer = TerminalRenderer::new()?;
    let colors = ColorScheme::warp();
    
    // Create sidebar
    let mut sidebar = Sidebar::new("Menu".to_string());
    sidebar.add_item("Option 1".to_string(), SidebarItemType::Workflow, 0);
    sidebar.add_item("Option 2".to_string(), SidebarItemType::Command, 1);
    
    // Render with styling
    let styled = colors.apply_style("My App", &colors.primary);
    renderer.write(&styled);
    renderer.flush()?;
    
    Ok(())
}
```

### Integrated Workflows + Themes

```rust
use warp_terminal_apps::{WarpTerminalUI, ConfigLoader};

fn main() -> std::io::Result<()> {
    let mut ui = WarpTerminalUI::new()?;
    
    // Load theme configuration
    let themes = ConfigLoader::load_themes_from_directory("terminal-apps/themes")?;
    
    // Initialize with theme
    let theme = themes.first().unwrap();
    ui.color_scheme = theme.to_color_scheme();
    
    ui.initialize_demo();
    ui.render()?;
    
    Ok(())
}
```

## Component Reference

### Sidebar

```rust
pub struct Sidebar {
    pub title: String,
    pub items: Vec<SidebarItem>,
    pub selected_index: usize,
}

pub enum SidebarItemType {
    Folder,
    Workflow,
    Command,
    Trash,
}
```

**Methods:**
- `new(title)` - Create sidebar
- `add_item(label, type, indent)` - Add menu item
- `select_next()` - Navigate down (vim: j)
- `select_prev()` - Navigate up (vim: k)

### FileList

```rust
pub struct FileList {
    pub items: Vec<FileItem>,
    pub selected_index: usize,
}

pub struct FileItem {
    pub name: String,
    pub size: String,
    pub date: String,
    pub kind: FileKind,
}

pub enum FileKind {
    File,
    Directory,
    Symlink,
}
```

**Methods:**
- `new()` - Create file list
- `add_file(name, size, date, kind)` - Add file
- `select_next()` - Move to next file
- `select_prev()` - Move to previous file

### TabBar

```rust
pub struct TabBar {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
}

pub struct Tab {
    pub title: String,
    pub path: String,
}
```

**Methods:**
- `new()` - Create tab bar
- `add_tab(title, path)` - Add new tab
- `select_tab(index)` - Switch to tab

### Color Schemes

```rust
pub struct ColorScheme {
    pub primary: Style,      // Cyan
    pub secondary: Style,    // Gray
    pub accent: Style,       // Cyan (bold)
    pub success: Style,      // Green
    pub error: Style,        // Red
    pub warning: Style,      // Yellow
    pub info: Style,         // Blue
    pub background: Style,   // White on dark
}
```

**Available Schemes:**
- `ColorScheme::warp()` - Warp terminal theme
- `ColorScheme::dark()` - Dark theme variant
- `ColorScheme::light()` - Light theme variant

## Keyboard Shortcuts

The UI supports vim-style navigation:

- **j** - Next item (down)
- **k** - Previous item (up)
- **Enter** - Select/Execute
- **Backspace** - Delete in command bar
- **Tab** - Switch pane focus
- **q** - Quit (in some apps)

## Styling Text

```rust
let colors = ColorScheme::warp();

// Apply primary style (cyan)
let styled = colors.apply_style("Important", &colors.primary);

// Apply error style (red)
let error = colors.apply_style("Error!", &colors.error);

// Apply success style (green)
let success = colors.apply_style("Success!", &colors.success);
```

## Integration with Terminal Apps Framework

The Terminal UI system fully integrates with the Warp terminal apps framework:

### 1. Theme Integration
```rust
let theme = ConfigLoader::load_theme("terminal-apps/themes/warp.yaml")?;
ui.apply_theme(&theme);
```

### 2. Keyset Integration
```rust
let keyset = ConfigLoader::load_keyset("terminal-apps/keysets/vim.yaml")?;
state.apply_keyset(&keyset);
```

### 3. Workflow Integration
```rust
let workflow = ConfigLoader::load_workflow("terminal-apps/workflows/build.yaml")?;
// Execute workflow and display progress in UI
```

## Examples

### Running Built-in Examples

```bash
# Warp Terminal UI demo
cargo run --example warp_ui

# Task Manager with custom UI
cargo run --example task_manager

# Build Monitor with UI
cargo run --example build_monitor

# Config Manager with UI
cargo run --example config_manager
```

## Performance Considerations

1. **Buffer Strategy**: Uses string buffering to batch terminal writes
2. **Minimal Redraws**: Only renders changed portions of screen
3. **Raw Mode**: Uses raw terminal mode for direct character handling
4. **No External TUI Frameworks**: Pure Rust implementation for speed and control

## Extending the UI

### Creating Custom Components

```rust
pub struct CustomComponent {
    title: String,
    data: Vec<String>,
}

impl CustomComponent {
    pub fn render(&self, renderer: &mut TerminalRenderer, colors: &ColorScheme) {
        let styled = colors.apply_style(&self.title, &colors.primary);
        renderer.write(&styled);
    }
}
```

### Handling Input Events

```rust
match key {
    'j' => state.sidebar.select_next(),
    'k' => state.sidebar.select_prev(),
    '\t' => state.focus_pane(next_pane),
    c if c.is_ascii_digit() => handle_number(c),
    _ => {}
}
```

## Troubleshooting

**Issue: Colors not displaying correctly**
- Ensure terminal supports 256 colors: `echo $TERM`
- Try setting `TERM=xterm-256color`

**Issue: Rendering artifacts**
- Call `renderer.flush()` after writes
- Ensure proper cursor positioning

**Issue: Raw mode not working**
- Check terminal supports raw mode
- Ensure proper cleanup on exit (Drop trait handles this)

## API Reference

See the source code in `terminal-apps/src/ui/` for complete documentation of:
- `TerminalRenderer` - Low-level rendering
- `Sidebar`, `FileList`, `TabBar` - Components
- `Layout` - Layout positioning
- `ColorScheme`, `Style` - Theming
- `UIState` - State management
