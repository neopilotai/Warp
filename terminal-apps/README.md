# Warp Terminal Apps Framework

A comprehensive framework for building interactive terminal applications with integrated themes, keysets, and workflow support.

## Features

### 🎨 Theme System
- **Configuration-based theming**: Load themes from YAML files
- **Color customization**: Full control over terminal colors and accents
- **Theme inheritance**: Create custom themes based on existing ones
- **Color palette management**: Separate normal and bright color variants

### ⌨️ Keyset Management
- **Pre-configured keysets**: Includes Vim, Emacs, VSCode-style bindings
- **Custom keybindings**: Define your own keyboard shortcuts
- **Keyset inheritance**: Build upon existing keysets
- **Dynamic binding lookup**: Get keybindings at runtime

### 🔄 Workflow Engine
- **Sequential workflows**: Execute steps in order
- **Conditional execution**: Branch based on variables and conditions
- **Error handling**: Define fallback steps on failure
- **Context variables**: Pass data between workflow steps
- **Pattern matching**: Support for exists, equals, contains, and glob patterns

## Module Structure

```
terminal-apps/
├── src/
│   ├── lib.rs           # Main library exports
│   ├── app.rs           # TerminalApp state management
│   ├── theme.rs         # Theme data structures and management
│   ├── keyset.rs        # Keyset and keybinding definitions
│   ├── workflow.rs      # Workflow engine with conditions
│   └── config_loader.rs # YAML/TOML configuration loading
└── examples/
    └── main.rs          # Example applications
```

## Quick Start

### 1. Creating a Terminal App

```rust
use warp_terminal_apps::{TerminalApp, Theme, KeySet};

let mut app = TerminalApp::new("MyApp");

// Register themes and keysets
app.register_theme(my_theme);
app.register_keyset(my_keyset);

// Set current configuration
app.set_theme("dark");
app.set_keyset("vim");
```

### 2. Working with Themes

```yaml
# my_theme.yaml
name: "dark"
background: "#1e1e1e"
foreground: "#e0e0e0"
accent: "#007acc"
details: "dark"
terminal_colors:
  normal:
    black: "#1e1e1e"
    red: "#f48771"
    green: "#6a9955"
    yellow: "#dcdcaa"
    blue: "#569cd6"
    magenta: "#c586c0"
    cyan: "#4ec9b0"
    white: "#e0e0e0"
  bright:
    black: "#666666"
    red: "#f48771"
    green: "#6a9955"
    yellow: "#dcdcaa"
    blue: "#569cd6"
    magenta: "#c586c0"
    cyan: "#4ec9b0"
    white: "#ffffff"
```

### 3. Defining Keysets

```yaml
# vim_keyset.yaml
name: "vim"
description: "Vim-style keybindings"
base: null
editor_view:save: "ctrl-s"
editor_view:undo: "ctrl-z"
terminal:find: "slash"
```

### 4. Building Workflows

```rust
use warp_terminal_apps::{ExtendedWorkflow, WorkflowStep, Condition};

let mut workflow = ExtendedWorkflow::new("deploy");
workflow.set_variable("environment", "production");

let build_step = WorkflowStep {
    name: "build".to_string(),
    workflow: "npm:build".to_string(),
    condition: None,
    on_success: Some(vec!["test".to_string()]),
    on_failure: Some(vec!["notify".to_string()]),
};

workflow.add_step(build_step);
```

### 5. Conditional Execution

```rust
use warp_terminal_apps::Condition;

let condition = Condition {
    variable: "environment".to_string(),
    operator: "equals".to_string(),
    value: Some("production".to_string()),
};

// Evaluate in a context
let mut context = ExecutionContext::new();
context.set_variable("environment", "production");
assert!(condition.evaluate(&context));
```

## Condition Operators

- **exists**: Check if a variable is set
- **equals**: Check if variable equals a value
- **contains**: Check if variable contains a substring
- **matches**: Check if variable matches a glob pattern

## Configuration Loading

```rust
use warp_terminal_apps::ConfigLoader;

// Load single theme
let theme = ConfigLoader::load_theme("path/to/theme.yaml")?;

// Load all themes from directory
let themes = ConfigLoader::load_themes_from_directory("themes/")?;

// Load keyset
let keyset = ConfigLoader::load_keyset("keysets/vim.yaml")?;

// Save custom configuration
ConfigLoader::save_theme(&my_theme, "custom_theme.yaml")?;
```

## Examples

Run the included examples:

```bash
cargo run --example main
```

This demonstrates:
1. **Task Runner**: Interactive app with themes and keybindings
2. **Workflow Builder**: Complex deployment workflow with conditions
3. **Configuration Manager**: Custom app settings

## Integration with Warp

The framework integrates seamlessly with Warp's existing:
- **Workflow types** (`warp-workflows-types`)
- **Theme library** (1000+ pre-configured themes)
- **Keyset collections** (Vim, Emacs, VSCode)

## Advanced Usage

### Custom Theme Extension

```rust
use warp_terminal_apps::Theme;
use std::collections::HashMap;

let mut theme = my_base_theme;
theme.custom_colors.insert("error".to_string(), "#ff0000".to_string());
theme.custom_colors.insert("success".to_string(), "#00ff00".to_string());
```

### Workflow Composition

Create reusable workflow templates:

```rust
fn create_ci_workflow() -> ExtendedWorkflow {
    let mut workflow = ExtendedWorkflow::new("ci-pipeline");
    // Add common steps
    workflow
}
```

### Error Handling

All functions return proper error types:

```rust
use warp_terminal_apps::{ThemeError, KeySetError, WorkflowError};

match ConfigLoader::load_theme("path") {
    Ok(theme) => { /* use theme */ }
    Err(ThemeError::NotFound(name)) => println!("Theme not found: {}", name),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Testing

The framework includes comprehensive tests:

```bash
cargo test
```

Tests cover:
- Theme creation and color lookup
- Keyset merging and binding resolution
- Workflow condition evaluation
- Configuration persistence
- Context variable management
