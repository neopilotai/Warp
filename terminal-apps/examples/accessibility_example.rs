// Accessibility Example: Building an Accessible Terminal App
// This example demonstrates how to build terminal applications using
// the terminal-apps framework with accessibility best practices.

use terminal_apps::{
    TerminalApp, Theme, KeySet, ExecutionContext, ExtendedWorkflow
};
use std::collections::HashMap;

/// Example: Building an accessible CLI application
/// Features demonstrated:
/// - Screen reader support through descriptions
/// - Fully customizable keybindings
/// - High contrast theme options
/// - Clear error messaging
/// - Accessible workflow descriptions

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Initialize the app with accessibility-focused setup
    let mut app = TerminalApp::new();
    
    // Step 2: Load themes with accessibility metadata
    let themes = load_accessible_themes();
    app.register_themes(themes);
    
    // Step 3: Load keysets optimized for different input methods
    let keysets = load_accessible_keysets();
    app.register_keysets(keysets);
    
    // Step 4: Set up accessible defaults
    setup_accessible_defaults(&mut app)?;
    
    // Step 5: Run accessible interactive CLI
    run_accessible_cli(&mut app)?;
    
    Ok(())
}

/// Load themes with accessibility metadata
/// Each theme includes WCAG contrast ratios and accessibility flags
fn load_accessible_themes() -> HashMap<String, Theme> {
    let mut themes = HashMap::new();
    
    // High Contrast Theme for visually impaired users
    themes.insert("high_contrast".to_string(), Theme {
        name: "high_contrast".to_string(),
        background: "#000000".to_string(),       // Pure black
        foreground: "#FFFFFF".to_string(),       // Pure white (21:1 contrast)
        accent: "#FFFF00".to_string(),           // Pure yellow (19.56:1 contrast)
        terminal_colors: Default::default(),
        custom_colors: {
            let mut custom = HashMap::new();
            custom.insert("contrast_ratio".to_string(), "21:1".to_string());
            custom.insert("wcag_level".to_string(), "AAA".to_string());
            custom.insert("description".to_string(), 
                "High contrast theme optimized for visual accessibility".to_string());
            custom
        },
    });
    
    // Standard Accessible Theme
    themes.insert("accessible_light".to_string(), Theme {
        name: "accessible_light".to_string(),
        background: "#FFFFFF".to_string(),
        foreground: "#000000".to_string(),       // 21:1 contrast
        accent: "#0066CC".to_string(),           // Blue with 8.59:1 contrast
        terminal_colors: Default::default(),
        custom_colors: {
            let mut custom = HashMap::new();
            custom.insert("contrast_ratio".to_string(), "21:1".to_string());
            custom.insert("wcag_level".to_string(), "AAA".to_string());
            custom
        },
    });
    
    themes
}

/// Load keysets optimized for different input methods
/// - vim_ergonomic: Reduced finger movement
/// - emacs_accessible: Familiar for traditional Unix users
/// - minimal_keyset: Fewest keys needed (for motor accessibility)
fn load_accessible_keysets() -> HashMap<String, KeySet> {
    let mut keysets = HashMap::new();
    
    // Ergonomic Vim variant for reduced hand strain
    let mut vim_ergonomic_bindings = HashMap::new();
    vim_ergonomic_bindings.insert("editor:save".to_string(), "ctrl-s".to_string());
    vim_ergonomic_bindings.insert("editor:undo".to_string(), "ctrl-z".to_string());
    vim_ergonomic_bindings.insert("editor:redo".to_string(), "ctrl-y".to_string());
    vim_ergonomic_bindings.insert("editor:find".to_string(), "ctrl-f".to_string());
    vim_ergonomic_bindings.insert("editor:replace".to_string(), "ctrl-h".to_string());
    // All Ctrl-key combinations reduce hand movement vs traditional Vim
    
    keysets.insert("vim_ergonomic".to_string(), KeySet {
        name: "vim_ergonomic".to_string(),
        base: Some("vim".to_string()),
        bindings: vim_ergonomic_bindings,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("description".to_string(), 
                "Ergonomic Vim variant with reduced finger movement".to_string());
            meta.insert("target_users".to_string(), "Motor accessibility".to_string());
            meta
        },
    });
    
    // Minimal keyset for users who need very few key combinations
    let mut minimal_bindings = HashMap::new();
    minimal_bindings.insert("menu:up".to_string(), "up".to_string());
    minimal_bindings.insert("menu:down".to_string(), "down".to_string());
    minimal_bindings.insert("menu:select".to_string(), "enter".to_string());
    minimal_bindings.insert("menu:cancel".to_string(), "esc".to_string());
    // Only 4 keys - minimal cognitive and motor load
    
    keysets.insert("minimal".to_string(), KeySet {
        name: "minimal".to_string(),
        base: None,
        bindings: minimal_bindings,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("description".to_string(), 
                "Minimal keyset with only essential arrow/enter/esc keys".to_string());
            meta.insert("keys_count".to_string(), "4".to_string());
            meta
        },
    });
    
    keysets
}

/// Set up accessible defaults
fn setup_accessible_defaults(app: &mut TerminalApp) -> Result<(), Box<dyn std::error::Error>> {
    // Set high contrast theme by default
    app.set_theme("high_contrast");
    
    // Set ergonomic keyset to reduce strain
    app.set_keyset("vim_ergonomic");
    
    // Enable verbose output for screen readers
    app.set_config("screen_reader_enabled", "true");
    app.set_config("verbose_descriptions", "true");
    
    // Disable animations that might cause issues for some users
    app.set_config("animations_enabled", "false");
    
    // Set font options for better readability
    app.set_config("font_size", "large");
    app.set_config("line_height", "comfortable");
    
    Ok(())
}

/// Demonstrate accessible CLI interaction
fn run_accessible_cli(app: &mut TerminalApp) -> Result<(), Box<dyn std::error::Error>> {
    // Print screen reader friendly header
    println!("=== Terminal Application ===");
    println!("Screen reader mode: ENABLED");
    println!("Current theme: {}", app.get_current_theme_name());
    println!("Current keyset: {}", app.get_current_keyset_name());
    println!("");
    
    // Accessibility: Describe available actions with full key combinations
    print_accessible_menu(&app);
    
    // Accessibility: Support multiple ways to interact
    demonstrate_workflow_with_accessibility(&app)?;
    
    Ok(())
}

/// Print menu with full descriptions for screen readers
fn print_accessible_menu(app: &TerminalApp) {
    println!("=== Available Actions ===");
    println!("");
    
    // List actions with keybindings
    let actions = vec![
        ("View Files", "menu:up"),
        ("Navigate Down", "menu:down"),
        ("Select Item", "menu:select"),
        ("Go Back", "menu:cancel"),
    ];
    
    for (action, keybind_id) in actions {
        if let Some(keys) = app.get_keybinding(keybind_id) {
            println!("Action: {}", action);
            println!("  Keyboard shortcut: {}", keys);
            println!("  Description: Press {} to {}", keys, action.to_lowercase());
            println!("");
        }
    }
}

/// Demonstrate accessible workflow execution
fn demonstrate_workflow_with_accessibility(app: &TerminalApp) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Starting Accessible Workflow ===");
    println!("");
    
    // Create a workflow with clear descriptions for each step
    let workflow = ExtendedWorkflow {
        name: "accessible_example".to_string(),
        steps: vec![],  // Simplified for example
        variables: {
            let mut vars = HashMap::new();
            vars.insert("environment".to_string(), "development".to_string());
            vars
        },
    };
    
    println!("Workflow: {}", workflow.name);
    println!("Environment: development");
    println!("");
    
    // For each step, provide clear feedback
    println!("Step 1: Validating configuration");
    println!("  Status: OK");
    println!("  Details: All required fields are present");
    println!("");
    
    println!("Step 2: Loading resources");
    println!("  Status: OK");
    println!("  Details: 5 resources loaded successfully");
    println!("");
    
    println!("Step 3: Executing main workflow");
    println!("  Status: IN PROGRESS");
    println!("  Details: Running checks...");
    println!("");
    
    // Accessibility: Provide status updates in a structured way
    println!("=== Workflow Status ===");
    println!("Progress: 3/5 steps complete");
    println!("Next action: Press ENTER to continue");
    println!("");
    
    Ok(())
}

/// Additional utility: Print keybinding help
fn print_keybinding_help(app: &TerminalApp) {
    println!("=== Keybinding Help ===");
    println!("Current keyset: {}", app.get_current_keyset_name());
    println!("");
    
    // List all available keybindings in the current keyset
    println!("Available keybindings:");
    let common_actions = vec![
        "editor:save", "editor:undo", "editor:redo",
        "editor:find", "editor:replace",
        "menu:up", "menu:down", "menu:select", "menu:cancel"
    ];
    
    for action in common_actions {
        if let Some(keys) = app.get_keybinding(action) {
            println!("  {} → {}", action, keys);
        }
    }
}

/// Helper: Get current theme name
trait TerminalAppExt {
    fn get_current_theme_name(&self) -> String;
    fn get_current_keyset_name(&self) -> String;
}

impl TerminalAppExt for TerminalApp {
    fn get_current_theme_name(&self) -> String {
        "high_contrast".to_string()  // Simplified
    }
    
    fn get_current_keyset_name(&self) -> String {
        "vim_ergonomic".to_string()   // Simplified
    }
}
