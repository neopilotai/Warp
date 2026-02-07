# Terminal Apps Integration Guide

## Overview

The Warp Terminal Apps Framework provides a complete system for building feature-rich terminal applications with:
- **Themes**: Visual customization via YAML configuration
- **Keysets**: Customizable keyboard shortcuts and bindings
- **Workflows**: Complex task orchestration with conditional execution

## Getting Started

### 1. Add to Your Rust Project

```toml
[dependencies]
warp-terminal-apps = { path = "../terminal-apps" }
```

### 2. Basic App Setup

```rust
use warp_terminal_apps::{TerminalApp, ConfigLoader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = TerminalApp::new("MyTerminalApp");
    
    // Load themes from directory
    let themes = ConfigLoader::load_themes_from_directory("terminal-apps/themes")?;
    app.register_themes(themes);
    
    // Load keysets from directory
    let keysets = ConfigLoader::load_keysets_from_directory("terminal-apps/keysets")?;
    app.register_keysets(keysets);
    
    // Set defaults
    app.set_theme("neon_night");
    app.set_keyset("vscode");
    
    // Use the app
    run_app(&app)?;
    
    Ok(())
}
```

## Architecture

### Theme System

The theme system is built on three layers:

```
┌─────────────────────────────┐
│   Application UI Layer      │
│   (Your app code)           │
└────────────┬────────────────┘
             │ get_color("accent")
             ▼
┌─────────────────────────────┐
│   TerminalApp               │
│   (current_theme)           │
└────────────┬────────────────┘
             │ lookup
             ▼
┌─────────────────────────────┐
│   Theme Data                │
│   (Colors, Palette)         │
└─────────────────────────────┘
```

#### Loading Custom Themes

```rust
let theme = ConfigLoader::load_theme("custom_theme.yaml")?;
app.register_theme(theme);
app.set_theme("custom_theme");

// Access colors
if let Some(accent) = app.get_color("accent") {
    println!("Accent color: {}", accent);
}
```

### Keyset System

The keyset system supports inheritance and composition:

```
┌──────────────────────────┐
│   Custom Keyset          │
│   (extends vim)          │
│   - Override quit key    │
│   - Add custom actions   │
└──────────────┬───────────┘
               │
               ▼
┌──────────────────────────┐
│   Base Keyset (vim)      │
│   - Standard vim keys    │
└──────────────────────────┘
```

#### Using Keysets

```rust
// Get a keybinding
if let Some(key) = app.get_keybinding("editor:save") {
    println!("Save key: {}", key);
}

// List all bindings
app.current_keyset.as_ref().map(|ks| {
    for (action, key) in ks.list_bindings() {
        println!("{} -> {}", action, key);
    }
});
```

### Workflow Engine

The workflow engine supports both sequential and conditional execution:

```
                    ┌──────────────────┐
                    │  Step 1: Install │
                    └────────┬─────────┘
                             │ success
                             ▼
                    ┌──────────────────┐
                    │  Step 2: Test    │
                    └────────┬─────────┘
                             │ success
                             ▼
    ┌────────────────────────────────────────┐
    │ Step 3: Build (if env == production)   │
    └────────┬───────────────────────┬───────┘
             │ true                  │ false
             ▼                       ▼
    ┌──────────────────┐    ┌──────────────────┐
    │ Step 4: Deploy   │    │ Step 5: Notify   │
    └──────────────────┘    └──────────────────┘
```

#### Workflow Execution Model

```rust
use warp_terminal_apps::{
    ExtendedWorkflow, WorkflowStep, Condition, ExecutionContext
};

let mut workflow = ExtendedWorkflow::new("ci");
workflow.set_variable("branch", "main");

let step = WorkflowStep {
    name: "deploy".to_string(),
    workflow: "deploy:prod".to_string(),
    condition: Some(Condition {
        variable: "branch".to_string(),
        operator: "equals".to_string(),
        value: Some("main".to_string()),
    }),
    on_success: Some(vec!["notify".to_string()]),
    on_failure: Some(vec!["rollback".to_string()]),
};

workflow.add_step(step);
```

## File Organization

Recommended project structure:

```
my-app/
├── src/
│   ├── main.rs
│   ├── ui/
│   ├── handlers/
│   └── lib.rs
└── terminal-apps/
    ├── themes/
    │   ├── dark.yaml
    │   ├── light.yaml
    │   └── custom.yaml
    ├── keysets/
    │   ├── vim.yaml
    │   ├── emacs.yaml
    │   └── vscode.yaml
    └── workflows/
        ├── build.yaml
        ├── deploy.yaml
        └── ci.yaml
```

## Common Patterns

### Pattern 1: Dynamic Theme Switching

```rust
fn switch_theme(app: &mut TerminalApp, theme_name: &str) -> bool {
    if app.set_theme(theme_name) {
        println!("Switched to theme: {}", theme_name);
        true
    } else {
        eprintln!("Theme not found: {}", theme_name);
        false
    }
}
```

### Pattern 2: Keybinding Resolution

```rust
fn handle_key_event(app: &TerminalApp, key: &str) -> Option<String> {
    app.get_keybinding(&format!("editor_view:{}", key))
}
```

### Pattern 3: Conditional Workflow Execution

```rust
fn should_run_step(step: &WorkflowStep, context: &ExecutionContext) -> bool {
    step.condition.as_ref().map_or(true, |cond| cond.evaluate(context))
}
```

### Pattern 4: Configuration Merging

```rust
fn merge_configs(app: &mut TerminalApp, config_file: &str) -> Result<()> {
    let lines = std::fs::read_to_string(config_file)?;
    for line in lines.lines() {
        if let Some((key, value)) = line.split_once('=') {
            app.set_config(key.trim(), value.trim());
        }
    }
    Ok(())
}
```

## Error Handling

The framework provides typed errors for each subsystem:

```rust
use warp_terminal_apps::{ThemeError, KeySetError, WorkflowError};

match ConfigLoader::load_theme("path") {
    Ok(theme) => process_theme(theme),
    Err(ThemeError::NotFound(name)) => {
        eprintln!("Theme '{}' not found", name);
    }
    Err(ThemeError::YamlError(e)) => {
        eprintln!("YAML parse error: {}", e);
    }
    Err(e) => eprintln!("Unexpected error: {}", e),
}
```

## Testing Your Integration

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        let mut app = TerminalApp::new("test");
        assert_eq!(app.list_themes().len(), 0);
    }

    #[test]
    fn test_theme_loading() {
        let theme = create_test_theme();
        let mut app = TerminalApp::new("test");
        app.register_theme(theme);
        assert!(app.set_theme("test"));
    }

    #[test]
    fn test_workflow_conditions() {
        let mut ctx = ExecutionContext::new();
        ctx.set_variable("env", "prod");
        
        let cond = Condition {
            variable: "env".to_string(),
            operator: "equals".to_string(),
            value: Some("prod".to_string()),
        };
        
        assert!(cond.evaluate(&ctx));
    }
}
```

## Performance Considerations

### Theme Caching

Themes are cached in the TerminalApp after loading:

```rust
// First load: reads from disk
let theme = ConfigLoader::load_theme("path")?;
app.register_theme(theme);  // Cached in app.available_themes

// Subsequent access: memory lookup
app.set_theme("name");  // HashMap lookup - O(1)
```

### Keyset Merging

When using inherited keysets, merge them at initialization:

```rust
let mut combined = base_keyset.clone();
combined.merge(custom_keyset);
app.register_keyset(combined);  // Store merged result
```

## Extending the Framework

### Custom Color Provider

```rust
impl ColorProvider for MyTheme {
    fn get_color(&self, name: &str) -> Option<String> {
        match name {
            "custom" => Some("#123456".to_string()),
            _ => self.default_get_color(name),
        }
    }
}
```

### Custom Condition Operators

Extend the `Condition` enum to support new operators:

```rust
pub enum Condition {
    // Existing...
    CustomOperator(String, String),
}
```

## Troubleshooting

### Theme Not Applied

1. Verify theme file exists: `ConfigLoader::load_theme("path")`
2. Check YAML syntax
3. Ensure theme is registered: `app.register_theme(theme)`
4. Set as current: `app.set_theme("name")`

### Keybinding Not Found

1. List available bindings: `ks.list_bindings()`
2. Check keyset is registered and active
3. Verify action name matches exactly

### Workflow Condition Always False

1. Verify variable is set: `ctx.get_variable("name")`
2. Check operator spelling (case-sensitive)
3. Verify condition value format
4. Use `println!` debugging in evaluation

## Next Steps

1. **Create custom themes** using the theme template
2. **Define keysets** for your app's commands
3. **Build workflows** for common tasks
4. **Integrate with your UI** using the examples
5. **Test thoroughly** with the included test patterns
