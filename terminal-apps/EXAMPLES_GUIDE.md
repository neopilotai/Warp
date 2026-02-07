# Terminal Apps Examples Guide

This directory contains production-ready example CLI applications that showcase the Warp Terminal Apps framework. Each example demonstrates real-world usage of themes, keysets, and workflows.

## Quick Start

Build all examples:
```bash
cd terminal-apps
cargo build --examples
```

Run a specific example:
```bash
cargo run --example task_manager
cargo run --example build_monitor
cargo run --example config_manager
```

---

## Example 1: Task Manager CLI

**Location**: `examples/task_manager.rs`

### Overview
An interactive todo list application that demonstrates theme-aware UI rendering and customizable keybindings.

### Features
- ✓ Add, complete, and remove tasks
- ✓ Priority levels (Low, Medium, High)
- ✓ Vim-style keybindings (j/k navigation, d delete, etc.)
- ✓ Color-coded priority indicators
- ✓ Real-time task status updates
- ✓ Interactive help system

### Architecture
```rust
struct Task {
    id: usize,
    title: String,
    completed: bool,
    priority: Priority,
}

struct TaskManager {
    app: TerminalApp,
    tasks: Vec<Task>,
    selected_index: usize,
    next_id: usize,
}
```

### Keybindings
- `j` - Move down
- `k` - Move up
- `a` - Add task
- `c` - Complete/uncomplete task
- `d` - Delete task
- `+` - Increase priority
- `-` - Decrease priority
- `?` - Show help
- `q` - Quit

### Theme Integration
The Task Manager uses a custom theme (`task_manager`) with:
- Dark background (#0f0f1e)
- Vibrant accent colors for priority levels
- High contrast for readability
- Terminal color palette for ANSI color support

### Use Cases
- Team task tracking
- Personal productivity tool
- Development sprint planning
- Example implementation for custom CLI tools

### Running the Example
```bash
cargo run --example task_manager
```

Then interact with:
- Press `a` to add a new task
- Use `j`/`k` to navigate
- Press `c` to mark complete
- Press `?` for help

---

## Example 2: Build Monitor

**Location**: `examples/build_monitor.rs`

### Overview
A sophisticated workflow executor that demonstrates conditional step execution, progress tracking, and real-time status indicators.

### Features
- ✓ Multi-step build pipeline execution
- ✓ Real-time progress indicators
- ✓ Color-coded status symbols (pending, running, success, failed, skipped)
- ✓ Performance timing metrics
- ✓ Error handling and failure recovery
- ✓ Conditional step execution
- ✓ Build summary and reports

### Architecture
```rust
#[derive(Debug, Clone, Copy)]
enum StepStatus {
    Pending,   // ⋯
    Running,   // ⟳
    Success,   // ✓
    Failed,    // ✗
    Skipped,   // ─
}

struct BuildStep {
    name: String,
    command: String,
    status: StepStatus,
    duration_ms: u128,
}

struct BuildMonitor {
    app: TerminalApp,
    steps: Vec<BuildStep>,
    current_step: usize,
    total_duration_ms: u128,
}
```

### Build Pipeline Steps
1. **cargo check** - Syntax checking (~1200ms)
2. **cargo build** - Compilation (~2500ms)
3. **cargo test** - Unit tests (~1800ms)
4. **cargo clippy** - Linting (~900ms)
5. **cargo doc** - Documentation (~500ms)

### Theme Integration
Uses `build_monitor` theme with:
- Professional dark background (#1a1a2e)
- Green for success states
- Red for failure states
- Yellow for warning states
- Cyan for running states

### Use Cases
- CI/CD pipeline monitoring
- Build system dashboards
- Test suite runners
- Deployment tracking
- Development workflow visualization

### Running the Example
```bash
cargo run --example build_monitor
```

The example simulates a complete build workflow with:
- Real-time step execution
- Progress bar updates
- Status indicators
- Final summary report

### Extending the Build Monitor
Add custom steps:
```rust
monitor.add_step("step_name".to_string(), "command_description".to_string());
```

---

## Example 3: Config Manager

**Location**: `examples/config_manager.rs`

### Overview
An interactive configuration explorer that demonstrates dynamic theme and keyset switching, configuration display, and settings management.

### Features
- ✓ Browse available themes and keysets
- ✓ Switch themes at runtime
- ✓ Switch keysets dynamically
- ✓ Display current configuration
- ✓ View theme color palettes
- ✓ Show keybinding mappings
- ✓ Export configuration as YAML
- ✓ Interactive menu navigation

### Architecture
```rust
struct ConfigManager {
    app: TerminalApp,
    available_themes: Vec<String>,
    available_keysets: Vec<String>,
    current_selection: usize,
}
```

### Menu Options
1. **View Available Themes** - List all registered themes
2. **View Available Keysets** - List all registered keysets
3. **Switch Theme** - Change active theme interactively
4. **Switch Keyset** - Change active keyset interactively
5. **View Current Configuration** - Show active theme and keyset details
6. **View Theme Colors** - Display color palette information
7. **View Keybindings** - Show all keybindings for current keyset
8. **Export Configuration** - Export settings as YAML

### Theme Management
The Config Manager can manage:
- `config_manager` - Main UI theme
- `task_manager` - Task management theme
- `build_monitor` - Build tracking theme
- `neon_night` - Custom preset theme

### Keyset Management
Supports multiple keybinding profiles:
- `emacs-config` - Emacs-style navigation
- `vim-tasks` - Vim-style bindings
- `monitor` - Build monitor controls

### Use Cases
- Settings management application
- User preference configuration
- Theme customization UI
- Keybinding configuration tool
- Configuration file generator

### Running the Example
```bash
cargo run --example config_manager
```

Interactive menu:
```
Select option: 1          # View themes
Select option: 3          # Switch to different theme
Select option: 6          # View colors
Select option: 8          # Export configuration
Select option: q          # Quit
```

### Configuration Export Format
Exported to `~/.config/warp/config.yaml`:
```yaml
theme: config_manager
keyset: emacs-config
```

---

## Other Examples

### main.rs
Comprehensive framework demonstration with:
- Task runner with themes
- Workflow builder with conditionals
- Configuration management

### interactive_cli.rs
Full-featured interactive application showing:
- Theme and keyset registration
- Workflow management
- Dynamic configuration
- Help system

### accessibility_example.rs
Accessibility-focused patterns:
- Screen reader compatibility
- High contrast themes
- Keyboard-only navigation
- Clear status announcements

---

## Framework Integration Patterns

### Pattern 1: Themes
```rust
let theme = Theme {
    name: "custom".to_string(),
    background: "#1e1e1e".to_string(),
    foreground: "#e0e0e0".to_string(),
    accent: "#007acc".to_string(),
    details: "dark".to_string(),
    terminal_colors: TerminalColors { /* ... */ },
    custom_colors: HashMap::new(),
};

app.register_theme(theme);
app.set_theme("custom");
```

### Pattern 2: Keysets
```rust
let mut keyset = KeySet::new("custom");
keyset.add_binding("action", "key_combo");

app.register_keyset(keyset);
app.set_keyset("custom");
```

### Pattern 3: Workflows
```rust
let mut workflow = ExtendedWorkflow::new("my_workflow");
workflow.set_variable("key", "value");
workflow.add_step(WorkflowStep { /* ... */ });
```

### Pattern 4: Configuration
```rust
app.set_config("setting_name", "value");
let config = &app.custom_config;
```

---

## Building Custom Examples

### Step 1: Create Your Example
```bash
touch examples/my_tool.rs
```

### Step 2: Add Binary to Cargo.toml
```toml
[[example]]
name = "my_tool"
path = "examples/my_tool.rs"
```

### Step 3: Use Framework
```rust
use warp_terminal_apps::{TerminalApp, Theme, KeySet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = TerminalApp::new("MyTool");
    
    // Initialize theme and keysets
    // Add your application logic
    
    Ok(())
}
```

### Step 4: Run It
```bash
cargo run --example my_tool
```

---

## Common Patterns & Tips

### Displaying Status
```rust
println!("╔════════════════════════╗");
println!("║  Application Status    ║");
println!("╠════════════════════════╣");
println!("║  Theme: {}    ║", theme_name);
println!("╚════════════════════════╝");
```

### Handling Input
```rust
let mut input = String::new();
io::stdin().read_line(&mut input)?;
match input.trim() {
    "j" => handle_down(),
    "k" => handle_up(),
    _ => println!("Unknown command"),
}
```

### Color-Coded Output
```rust
let status = StepStatus::Success;
println!("{}{}✓ Completed\x1b[0m", status.color_code(), status.symbol());
```

### Conditional Logic
```rust
if condition.evaluate(&context) {
    println!("Condition met!");
    // Execute success path
} else {
    println!("Condition failed!");
    // Execute failure path
}
```

---

## Performance Considerations

1. **Memory**: Each theme stores multiple color palettes - consider lazy loading
2. **I/O**: Config loading happens on startup - cache after initial load
3. **UI Rendering**: Limit redraws to only changed elements
4. **Workflows**: Long-running steps should provide progress updates

---

## Testing Examples

Run all examples:
```bash
cargo test --examples
```

Test specific example:
```bash
cargo run --example task_manager -- --test
```

---

## Contributing New Examples

When adding new examples:
1. Follow naming convention: `your_tool_name.rs`
2. Include doc comments explaining features
3. Add to `Cargo.toml` `[[example]]` section
4. Document in this guide
5. Include real-world use cases
6. Show theme and keyset integration

---

## Resources

- [Terminal Apps README](./README.md) - Framework overview
- [Module Documentation](../MODULE_DOCUMENTATION.md) - API reference
- [Integration Guide](../INTEGRATION_GUIDE.md) - Integration patterns
- [Architecture](../ARCHITECTURE.md) - System design

---

## Troubleshooting

**Example won't compile?**
- Ensure all dependencies are in `Cargo.toml`
- Check that the example path is correct
- Verify module visibility in `src/lib.rs`

**Theme not applying?**
- Verify theme is registered before setting
- Check theme name spelling
- Ensure terminal supports the colors

**Keybindings not working?**
- Confirm keyset is registered
- Check for key conflict with shell
- Verify keyset name is active

---

## License

Part of the Warp Terminal Apps Framework. See LICENSE for details.
