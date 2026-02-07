# Warp Terminal Apps - Quick Reference

## 30-Second Overview

The Warp Terminal Apps Framework provides:
- **Themes**: YAML-based color schemes with 16-color palettes
- **Keysets**: Customizable keyboard shortcuts with inheritance
- **Workflows**: Task orchestration with conditional branching

## Essential Imports

```rust
use warp_terminal_apps::{
    TerminalApp,           // Main app container
    Theme, KeySet,         // Config types
    ExtendedWorkflow, WorkflowStep, Condition, ExecutionContext,
    ConfigLoader,          // File loading/saving
};
```

## Core Workflow

```rust
// 1. Create app
let mut app = TerminalApp::new("MyApp");

// 2. Load config
let themes = ConfigLoader::load_themes_from_directory("terminal-apps/themes/")?;
let keysets = ConfigLoader::load_keysets_from_directory("terminal-apps/keysets/")?;

// 3. Register
app.register_themes(themes);
app.register_keysets(keysets);

// 4. Activate
app.set_theme("dark");
app.set_keyset("vim");

// 5. Use
if let Some(color) = app.get_color("accent") { /* ... */ }
if let Some(key) = app.get_keybinding("editor:save") { /* ... */ }
```

## Creating Themes

### Programmatically
```rust
let theme = Theme {
    name: "dark".to_string(),
    background: "#000000".to_string(),
    foreground: "#FFFFFF".to_string(),
    accent: "#FF0000".to_string(),
    details: "dark".to_string(),
    terminal_colors: TerminalColors {
        normal: ColorPalette { /* ... */ },
        bright: ColorPalette { /* ... */ },
    },
    custom_colors: HashMap::new(),
};
```

### Via YAML
```yaml
name: "dark"
background: "#000000"
foreground: "#FFFFFF"
accent: "#FF0000"
details: "dark"
terminal_colors:
  normal:
    black: "#1c1c1c"
    red: "#bc5653"
    green: "#909d63"
    # ... 5 more colors
  bright:
    black: "#636363"
    red: "#bc5653"
    # ... 7 more colors
```

## Creating Keysets

### Programmatically
```rust
let mut keyset = KeySet::new("vim");
keyset.add_binding("save", "ctrl-s");
keyset.add_binding("quit", "ctrl-q");
```

### Via YAML
```yaml
name: "vim"
description: "Vim keybindings"
editor_view:save: "ctrl-s"
editor_view:quit: "ctrl-q"
editor_view:undo: "ctrl-z"
```

## Building Workflows

### Simple Sequential
```rust
let mut workflow = ExtendedWorkflow::new("build");

let step1 = WorkflowStep {
    name: "compile".to_string(),
    workflow: "cargo:build".to_string(),
    condition: None,
    on_success: Some(vec!["test".to_string()]),
    on_failure: Some(vec!["notify".to_string()]),
};

workflow.add_step(step1);
```

### With Conditions
```rust
let condition = Condition {
    variable: "env".to_string(),
    operator: "equals".to_string(),
    value: Some("production".to_string()),
};

let step = WorkflowStep {
    name: "deploy".to_string(),
    workflow: "deploy:prod".to_string(),
    condition: Some(condition),
    on_success: None,
    on_failure: Some(vec!["rollback".to_string()]),
};
```

## Condition Operators

```rust
// exists - check if variable is set
Condition {
    variable: "FILE".to_string(),
    operator: "exists".to_string(),
    value: None,
}

// equals - exact match
Condition {
    variable: "ENV".to_string(),
    operator: "equals".to_string(),
    value: Some("prod".to_string()),
}

// contains - substring
Condition {
    variable: "VERSION".to_string(),
    operator: "contains".to_string(),
    value: Some("beta".to_string()),
}

// matches - glob pattern (*, ?)
Condition {
    variable: "FILE".to_string(),
    operator: "matches".to_string(),
    value: Some("*.rs".to_string()),
}
```

## Configuration Management

```rust
// Set config
app.set_config("log_level", "debug");
app.set_config("timeout", "30");

// Get config
if let Some(level) = app.get_config("log_level") {
    println!("Log level: {}", level);
}

// List all
for (key, value) in &app.custom_config {
    println!("{} = {}", key, value);
}
```

## File Operations

```rust
// Load
let theme = ConfigLoader::load_theme("themes/dark.yaml")?;
let keyset = ConfigLoader::load_keyset("keysets/vim.yaml")?;

// Load all from directory
let themes = ConfigLoader::load_themes_from_directory("themes/")?;
let keysets = ConfigLoader::load_keysets_from_directory("keysets/")?;

// Save
ConfigLoader::save_theme(&my_theme, "custom.yaml")?;
ConfigLoader::save_keyset(&my_keyset, "custom.yaml")?;
```

## Error Handling

```rust
use warp_terminal_apps::{ThemeError, KeySetError, WorkflowError};

match ConfigLoader::load_theme("path") {
    Ok(theme) => { /* use theme */ }
    Err(ThemeError::NotFound(name)) => eprintln!("Not found: {}", name),
    Err(ThemeError::YamlError(e)) => eprintln!("Parse error: {}", e),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Common Patterns

### Switch Theme on Demand
```rust
fn set_theme(app: &mut TerminalApp, name: &str) -> Result<(), String> {
    if app.set_theme(name) {
        Ok(())
    } else {
        Err(format!("Theme {} not found", name))
    }
}
```

### Get All Available Options
```rust
fn list_options(app: &TerminalApp) {
    println!("Themes: {:?}", app.list_themes());
    println!("Keysets: {:?}", app.list_keysets());
}
```

### Apply Configuration from File
```rust
fn load_config_file(app: &mut TerminalApp, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    for line in content.lines() {
        if let Some((k, v)) = line.split_once('=') {
            app.set_config(k.trim(), v.trim());
        }
    }
    Ok(())
}
```

### Execute Workflow with Conditions
```rust
fn should_run_step(step: &WorkflowStep, ctx: &ExecutionContext) -> bool {
    step.condition.as_ref()
        .map_or(true, |cond| cond.evaluate(ctx))
}
```

## Project Structure

```
my-project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── themes/
│   ├── dark.yaml
│   ├── light.yaml
│   └── custom.yaml
├── keysets/
│   ├── vim.yaml
│   ├── emacs.yaml
│   └── vscode.yaml
├── workflows/
│   ├── build.yaml
│   ├── deploy.yaml
│   └── ci.yaml
└── Cargo.toml
```

## Cargo.toml Setup

```toml
[dependencies]
warp-terminal-apps = { path = "../terminal-apps" }
serde_yaml = "0.9"

[dev-dependencies]
tempfile = "3.8"  # For testing
```

## Running Examples

```bash
# Framework overview
cargo run --example main

# Interactive CLI demo
cargo run --example interactive_cli

# Run tests
cargo test

# Generate docs
cargo doc --open
```

## Data Flow

```
User Action
    ↓
TerminalApp (state machine)
    ├─ current_theme
    ├─ current_keyset
    ├─ available_themes (HashMap)
    ├─ available_keysets (HashMap)
    └─ custom_config (HashMap)
    ↓
Rendering Layer
    ├─ app.get_color("name")
    └─ app.get_keybinding("action")
    ↓
UI Output
```

## Type Relationships

```
Theme
  ├─ TerminalColors
  │   ├─ normal: ColorPalette
  │   └─ bright: ColorPalette
  └─ custom_colors: HashMap

KeySet
  └─ bindings: HashMap<action, key>

ExtendedWorkflow
  ├─ steps: Vec<WorkflowStep>
  └─ variables: HashMap

WorkflowStep
  ├─ condition: Option<Condition>
  ├─ on_success: Option<Vec<String>>
  └─ on_failure: Option<Vec<String>>

Condition
  └─ operator: "exists" | "equals" | "contains" | "matches"

ExecutionContext
  └─ variables: HashMap<key, value>
```

## Common Tasks

| Task | Code |
|------|------|
| Create app | `TerminalApp::new("name")` |
| Load themes | `ConfigLoader::load_themes_from_directory("dir")?` |
| Set theme | `app.set_theme("name")` |
| Get color | `app.get_color("accent")` |
| Add keyset | `app.register_keyset(keyset)` |
| Get keybinding | `app.get_keybinding("action")` |
| Create workflow | `ExtendedWorkflow::new("name")` |
| Add step | `workflow.add_step(step)` |
| Check condition | `condition.evaluate(&context)` |
| Save config | `ConfigLoader::save_theme(&theme, "path")` |

## Debugging Tips

```rust
// Debug theme lookup
println!("[DEBUG] Theme: {:?}", app.current_theme.as_ref().map(|t| &t.name));

// Debug keybinding resolution
println!("[DEBUG] Available bindings: {:?}", 
    app.current_keyset.as_ref().map(|k| k.list_bindings()));

// Debug workflow structure
println!("[DEBUG] Workflow steps: {}", workflow.steps.len());
for step in &workflow.steps {
    println!("  - {}: {:?}", step.name, step.condition);
}

// Debug condition evaluation
let result = condition.evaluate(&context);
println!("[DEBUG] Condition result: {}", result);
```

## Performance Notes

- Theme lookup: O(1) - HashMap access
- Keybinding lookup: O(1) - HashMap access  
- Theme switching: O(1) - HashMap access
- Workflow validation: O(n) - Linear steps
- Condition evaluation: O(1) - Single operation
- Directory scan: O(files) - I/O bound

---

**For detailed documentation, see:**
- `README.md` - Complete API reference
- `INTEGRATION_GUIDE.md` - Integration patterns
- `examples/main.rs` - Runnable demonstrations
