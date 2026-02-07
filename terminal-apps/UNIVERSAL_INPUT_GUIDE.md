# Universal Input System Guide

Complete documentation for Warp's Universal Input system integrated into the Terminal Apps framework.

## Overview

The Universal Input system provides five core components that work together to create an intelligent, context-aware terminal input experience:

1. **Advanced Input Component** - Mode-aware input with syntax highlighting
2. **Contextual Chips** - Display relevant context (directory, Git, conversations, etc.)
3. **Input Toolbelt** - Quick-access controls for input features (@context, /commands, voice, etc.)
4. **Mode Detector** - Automatic Terminal/Agent mode detection
5. **Smart Features** - Autocomplete, suggestions, error detection

## Component Details

### 1. Advanced Input Component

Handles text input with support for multiple modes and syntax highlighting.

```rust
use warp_terminal_apps::*;

let mut input = AdvancedInput::new();

// Insert text
input.insert_char('l');
input.insert_char('s');

// Navigate
input.move_cursor_left();
input.move_cursor_end();

// Syntax highlighting
let highlighted = input.get_highlighted_lines();

// Mode switching
input.set_mode(InputMode::Agent);
input.set_mode(InputMode::Terminal);
input.set_mode(InputMode::Auto);

// History navigation
input.add_to_history();
input.history_previous();
input.history_next();
```

**Features:**
- Character-by-character input with cursor tracking
- Vim-style cursor movement
- Syntax highlighting for shell commands
- Input history with navigation
- Mode switching between Terminal, Agent, and Auto

### 2. Contextual Chips

Display contextual information as interactive chips.

```rust
use warp_terminal_apps::*;

let mut chips = ContextualChips::new();

// Add directory
chips.add_directory_chip("/home/user/projects".to_string());

// Add Git info
chips.add_git_chip("main".to_string(), GitStatus::Clean);

// Add conversation reference
chips.add_conversation_chip("conv-abc123".to_string());

// Add attachment
chips.add_attachment_chip("config.yaml".to_string());

// Add runtime version
chips.add_runtime_chip("Node.js".to_string(), "18.0.0".to_string());

// Display all chips
println!("{}", chips.get_display_text());
// Output: 📁 Directory: /home/user/projects  ⎇ Git: main (✓)  💬 Conversation: conv-abc123  ...
```

**Chip Types:**
- `Directory` - Current working directory
- `GitStatus` - Git branch and repository status (Clean/Modified/Untracked/Mixed)
- `Conversation` - Active conversation reference
- `Attachment` - File attachments
- `RuntimeVersion` - Programming language/runtime versions
- `Profile` - Active execution profile
- `Custom` - User-defined chip types

### 3. Input Toolbelt

Quick-access controls for input features.

```rust
use warp_terminal_apps::*;

let mut toolbelt = InputToolbelt::new();

// Navigate toolbelt
toolbelt.select_next();
toolbelt.select_prev();
toolbelt.select_item(0);

// Get selected item
if let Some(item) = toolbelt.get_selected_item() {
    println!("{}", item.display());
}

// Find by hotkey
if let Some(item) = toolbelt.find_item_by_hotkey("Ctrl+@") {
    // Use @-context feature
}

// Toggle visibility
toolbelt.toggle_visibility();

// Display all items
for item in toolbelt.get_display_items() {
    println!("{}", item);
}
```

**Default Toolbelt Items:**
- `@` Context - Add context with @-mentions (Ctrl+@)
- `/` Commands - Quick access to slash commands (Ctrl+/)
- `🎤` Voice - Voice-to-text input (Ctrl+M)
- `📎` Attachments - Attach files (Ctrl+K)
- `⚡` FastForward - Quick execution modes (Ctrl+E)
- `👤` Profile - Switch execution profiles (Ctrl+P)
- `🤖` Models - Select AI model for Agent mode (Ctrl+L)

### 4. Mode Detector

Automatically detects whether input is a shell command or AI prompt.

```rust
use warp_terminal_apps::*;

let detector = ModeDetector::new();

// Detect mode
let mode = detector.detect("ls -la");  // Returns: DetectedMode::Terminal
let mode = detector.detect("How do I list files?");  // Returns: DetectedMode::Agent

// Get confidence
let confidence = detector.get_confidence("ls -la");  // 0.0-1.0

// Detailed analysis
let analysis = detector.analyze("find . -name *.rs");
println!("Mode: {:?}", analysis.detected);
println!("Confidence: {:.2}", analysis.confidence);
println!("Terminal Score: {:.2}", analysis.terminal_score);
println!("Agent Score: {:.2}", analysis.agent_score);
println!("Reasoning: {}", analysis.reasoning);
```

**Detection Algorithm:**
- Looks for shell operators: `|`, `>`, `<`, `>>`, `&&`, `||`
- Matches against terminal keywords: `ls`, `grep`, `git`, `cd`, etc.
- Matches against agent keywords: `help`, `explain`, `generate`, etc.
- Detects question marks and natural language indicators
- Calculates confidence scores based on token matches

### 5. Smart Features

Autocomplete, suggestions, and error detection.

```rust
use warp_terminal_apps::*;

let mut features = SmartFeatures::new();

// Get suggestions
let suggestions = features.get_suggestions("gr");
for suggestion in suggestions {
    println!("{}: {}", suggestion.text, suggestion.description);
}

// Check for errors
let errors = features.check_input("echo \"hello");
for error in errors {
    println!("{}: {}", error.message, error.severity);
}

// Add custom command
features.auto_completion.add_command(
    "mycommand".to_string(),
    "My custom command".to_string(),
);

// Toggle features
features.toggle();
```

**AutoCompletion:**
- Command suggestions with descriptions
- File/directory completion
- History-based suggestions
- Priority-based ranking

**Error Detection:**
- Unmatched quotes detection
- Unmatched parentheses detection
- Typo detection and suggestions
- Deprecated command warnings

## Integration with Terminal UI

The Universal Input system integrates seamlessly with the Terminal UI:

```rust
use warp_terminal_apps::*;

// Create UI state
let mut ui_state = UIState::new();

// Create universal input
let mut universal_input = UniversalInput {
    input: AdvancedInput::new(),
    chips: ContextualChips::new(),
    toolbelt: InputToolbelt::new(),
    mode_detector: ModeDetector::new(),
    smart_features: SmartFeatures::new(),
};

// Update command bar with universal input
ui_state.command_bar.input = universal_input.input.content.clone();
```

## Usage Examples

### Example 1: Terminal Command Execution

```rust
let detector = ModeDetector::new();
let mut input = AdvancedInput::new();

// User types: "git status"
for ch in "git status".chars() {
    input.insert_char(ch);
}

let mode = detector.detect(&input.content);
assert_eq!(mode, DetectedMode::Terminal);
```

### Example 2: Agent Prompt Processing

```rust
let detector = ModeDetector::new();

// User types: "How do I commit changes?"
let mode = detector.detect("How do I commit changes?");
assert_eq!(mode, DetectedMode::Agent);
```

### Example 3: Context-Aware Workflow

```rust
let mut chips = ContextualChips::new();
let mut features = SmartFeatures::new();

// Set up context
chips.add_directory_chip("/project".to_string());
chips.add_git_chip("develop".to_string(), GitStatus::Modified);

// Get relevant suggestions
let suggestions = features.get_suggestions("git");
// Returns: [commit, branch, checkout, etc.]

// Display context
println!("Working in: {}", chips.get_display_text());
```

## Running the Demo

To see all five components in action:

```bash
cd terminal-apps
cargo run --example universal_input_demo
```

Output shows:
1. Advanced input with mode switching
2. Contextual chips in action
3. Toolbelt items and hotkeys
4. Mode detection with confidence scores
5. Autocomplete suggestions and error detection

## Architecture

```
UniversalInput
├── AdvancedInput (advanced_input.rs)
│   ├── InputMode (Terminal/Agent/Auto)
│   ├── SyntaxHighlighting
│   └── History management
├── ContextualChips (contextual_chips.rs)
│   ├── Chip types (Directory, Git, Conversation, etc.)
│   └── GitInfo with status
├── InputToolbelt (input_toolbelt.rs)
│   ├── ToolbeltItem types
│   └── Default items with hotkeys
├── ModeDetector (mode_detector.rs)
│   ├── Terminal keyword matching
│   ├── Agent keyword matching
│   └── Confidence scoring
└── SmartFeatures (smart_features.rs)
    ├── AutoCompletion
    ├── ErrorDetector
    └── Suggestion ranking
```

## Performance Considerations

- Mode detection: O(n) where n is token count
- Suggestions: Limited to top 10 results by default
- History: Capped at 1000 entries
- Chips: Limited to 8 visible chips (oldest removed)

## Future Enhancements

- Plugin system for custom mode detectors
- Machine learning-based mode detection
- Context-aware command suggestions
- Integration with clipboard/system APIs
- Streaming input handling
- Multi-line command support
- Custom keybinding system
