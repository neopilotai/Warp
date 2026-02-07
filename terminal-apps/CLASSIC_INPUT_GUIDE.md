# Classic Input Guide

## Overview

Classic Input is Warp's traditional terminal-style input interface. It provides a familiar shell-like experience while maintaining modern terminal features and Agent Mode support. This guide covers implementing and using Classic Input in your terminal applications.

## Key Concepts

### Input Modes

Classic Input supports multiple input modes:
- **Terminal Mode**: Traditional shell command entry
- **Agent Mode**: Natural language query interface
- **Auto-detection**: Automatic switching based on input type

### Core Components

#### 1. Prompt System

The `Prompt` struct manages command-line prompts with multiple styles:

```rust
use warp_terminal_apps::classic_input::{ClassicInput, PromptStyle, Prompt};

// Warp-style prompt: user@host dir $ 
let prompt_warp = Prompt::new(PromptStyle::Warp);

// Traditional shell prompt: user@host:dir$
let prompt_ps1 = Prompt::new(PromptStyle::Shell);

// Custom prompt template
let prompt_custom = Prompt::new(PromptStyle::Custom("my-prompt> ".to_string()));
```

#### 2. Text Editor

The `ClassicEditor` provides modern text editing capabilities:

```rust
let mut editor = ClassicEditor::new();

// Insert and navigate
editor.insert_char('l');
editor.insert_char('s');
editor.move_cursor_left();
editor.delete_forward();

// Cursor management
editor.move_cursor_home();
editor.move_cursor_end();
editor.clear_line();
```

#### 3. Command History

`CommandHistory` maintains a scrollable history of executed commands:

```rust
let mut history = CommandHistory::new(1000); // Max 1000 commands

history.add("ls -la".to_string());
history.add("cd projects".to_string());

// Navigate history
if let Some(cmd) = history.previous() {
    println!("Previous: {}", cmd);
}

// Search history
let matching = history.search("ls");
```

#### 4. Agent Mode

`AgentMode` handles natural language detection and AI integration:

```rust
let mut agent = AgentMode::new();

// Check for natural language
agent.check_natural_language("list all python files");
if agent.is_active() {
    println!("Detected natural language query");
}

// Control auto-detection
agent.enable_auto_detection();
agent.disable_auto_detection();

// Manage denylists
agent.add_to_denylist("ls".to_string());
agent.remove_from_denylist("ls");
```

#### 5. Text Selection

`TextSelection` supports smart and rectangular (column) selection:

```rust
use warp_terminal_apps::classic_input::{TextSelection, SelectionMode};

let mut selection = TextSelection::new();

selection.start_selection(0);
selection.extend_selection(10);

// Toggle between selection modes
selection.toggle_mode();

// Get selected text
let selected = selection.get_selected_text("some text");
```

## Complete Example

```rust
use warp_terminal_apps::classic_input::ClassicInput;

fn main() {
    let mut input = ClassicInput::new();
    
    // Configure prompt
    input.prompt = Prompt::new(PromptStyle::Warp);
    
    // Handle text input
    input.handle_input('l');
    input.handle_input('s');
    
    // Get current input line
    let line = input.render_input_line();
    println!("{}", line);
    
    // Submit command (saves to history)
    let command = input.submit_command();
    println!("Executed: {}", command);
    
    // Navigate history
    input.navigate_history_prev();
    
    // Agent Mode
    input.toggle_agent_mode();
    if input.agent_mode.is_active() {
        println!("Agent Mode active");
    }
    
    // Input hints
    if let Some(hint) = input.get_input_hint() {
        println!("Hint: {}", hint);
    }
}
```

## Agent Mode Features

### Natural Language Auto-Detection

Classic Input automatically detects natural language queries using keyword matching and command analysis:

```rust
// These trigger Agent Mode
agent.check_natural_language("what are all the python files");  // Detected
agent.check_natural_language("find config files");              // Detected
agent.check_natural_language("ls -la");                         // Not detected
agent.check_natural_language("grep pattern file");              // Not detected
```

### Denylist Management

Prevent false positives by denylisting commands:

```rust
agent.add_to_denylist("show".to_string());  // "show" won't trigger Agent Mode
```

### Auto-detection Control

```rust
agent.enable_auto_detection();   // Enable local NL detection
agent.disable_auto_detection();  // Disable NL detection
```

## Input Hints

Contextual hints guide users through available features:

```rust
input.enable_input_hints();
input.disable_input_hints();

if let Some(hint) = input.get_input_hint() {
    println!("💡 {}", hint);
}
```

Available hints:
- Empty input: "Type a command or natural language query..."
- Agent Mode active: "Press ENTER to send query to AI, or ESC to cancel"

## Text Selection

### Smart Selection

Smart selection intelligently selects words or lines based on cursor position.

### Rectangular Selection

Rectangular (column) selection for precise multi-line highlighting:

```rust
selection.toggle_mode();  // Switch to rectangular mode
selection.start_selection(row, col);
selection.extend_selection(row, col);
```

## Integration with Warp Terminal UI

Classic Input integrates seamlessly with the Warp Terminal UI system:

```rust
use warp_terminal_apps::{ClassicInput, WarpTerminalUI};

let input = ClassicInput::new();
let mut ui = WarpTerminalUI::new();

// Render classic input in the UI
let rendered = input.render_input_line();
```

## Comparison: Classic Input vs Universal Input

| Feature | Classic Input | Universal Input |
|---------|---------------|-----------------|
| Terminal Experience | Traditional | Modern |
| Shell Customization | Full (PS1, oh-my-zsh) | Limited |
| Agent Mode | Yes | Yes |
| Contextual Chips | No | Yes |
| Input Toolbelt | No | Yes |
| Text Editing | Modern IDE-style | Enhanced |

## Keyboard Shortcuts

| Action | Keys |
|--------|------|
| Enter Agent Mode | Cmd+I or `*` + Space |
| Exit Agent Mode | ESC or Ctrl+C |
| Cursor Home | Home or Ctrl+A |
| Cursor End | End or Ctrl+E |
| Previous Command | ↑ |
| Next Command | ↓ |
| Select Mode | Shift+Arrow |

## Running the Demo

```bash
cd terminal-apps
cargo run --example classic_input_demo
```

This demonstrates:
- Prompt style rendering
- Text editing operations
- Command history navigation
- Natural language detection
- Input hints
- Text selection modes

## Best Practices

1. **Always track command history** for better UX
2. **Configure denylists** to prevent auto-detection false positives
3. **Use appropriate prompt styles** for your shell environment
4. **Enable input hints** for first-time users
5. **Support Agent Mode** for natural language queries

## Architecture

```
ClassicInput (main interface)
├── ClassicEditor (text editing)
├── Prompt (command prompt rendering)
├── CommandHistory (command tracking)
├── AgentMode (natural language detection)
├── TextSelection (text selection)
└── InputHints (contextual help)
```

Each component is independent and can be used separately in custom implementations.
