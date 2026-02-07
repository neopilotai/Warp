use warp_terminal_apps::*;

fn main() {
    println!("=== Universal Input System Demo ===\n");

    // 1. Advanced Input Component
    demo_advanced_input();

    // 2. Contextual Chips
    demo_contextual_chips();

    // 3. Input Toolbelt
    demo_input_toolbelt();

    // 4. Mode Detector
    demo_mode_detector();

    // 5. Smart Features
    demo_smart_features();
}

fn demo_advanced_input() {
    println!("1. Advanced Input Component");
    println!("---");

    let mut input = AdvancedInput::new();
    
    // Demonstrate input manipulation
    for ch in "ls -la /tmp".chars() {
        input.insert_char(ch);
    }
    println!("Input: {}", input.content);
    println!("Cursor at: {}", input.cursor_position);

    // Switch mode
    input.set_mode(InputMode::Agent);
    println!("Mode: {:?}", input.mode);

    // Add to history
    input.add_to_history();
    input.clear();

    // New input
    for ch in "pwd".chars() {
        input.insert_char(ch);
    }
    input.add_to_history();

    // Navigate history
    input.history_previous();
    println!("From history: {}\n", input.content);
}

fn demo_contextual_chips() {
    println!("2. Contextual Chips System");
    println!("---");

    let mut chips = ContextualChips::new();

    // Add directory chip
    chips.add_directory_chip("/home/developer/projects".to_string());

    // Add Git chip
    chips.add_git_chip("main".to_string(), GitStatus::Clean);

    // Add conversation chip
    chips.add_conversation_chip("conv-12345".to_string());

    // Add attachment
    chips.add_attachment_chip("config.yaml".to_string());

    // Add runtime
    chips.add_runtime_chip("Node.js".to_string(), "v18.0.0".to_string());

    println!("Active chips: {}", chips.get_display_text());
    println!("Total chips: {}\n", chips.chips.len());
}

fn demo_input_toolbelt() {
    println!("3. Input Toolbelt");
    println!("---");

    let toolbelt = InputToolbelt::new();

    println!("Available toolbelt items:");
    for (i, item) in toolbelt.get_display_items().iter().enumerate() {
        println!("  {}. {}", i + 1, item);
    }

    println!("\nHotkey lookup:");
    if let Some(item) = toolbelt.find_item_by_hotkey("Ctrl+@") {
        println!("  Ctrl+@ → {}\n", item.label);
    }
}

fn demo_mode_detector() {
    println!("4. Mode Detector (Natural Language Detection)");
    println!("---");

    let detector = ModeDetector::new();

    // Test cases
    let inputs = vec![
        "ls -la /home",
        "cat file.txt | grep pattern",
        "How do I list files?",
        "Explain what grep does",
        "find . -name *.rs -type f",
    ];

    for input in inputs {
        let analysis = detector.analyze(input);
        println!("Input: \"{}\"", input);
        println!("  Detected: {:?}", analysis.detected);
        println!("  Confidence: {:.2}", analysis.confidence);
        println!("  Reason: {}\n", analysis.reasoning);
    }
}

fn demo_smart_features() {
    println!("5. Smart Features (Autocomplete & Error Detection)");
    println!("---");

    let mut features = SmartFeatures::new();

    // Autocomplete
    println!("Autocompletion suggestions for 'gr':");
    let suggestions = features.get_suggestions("gr");
    for (i, suggestion) in suggestions.iter().take(3).enumerate() {
        println!("  {}. {} - {}", i + 1, suggestion.text, suggestion.description);
    }

    // Error detection
    println!("\nError checking for 'echo \"hello':");
    let errors = features.check_input("echo \"hello");
    for error in errors {
        println!("  ⚠ {}", error.message);
    }

    println!();
}

// Helper enum for GitStatus (would normally import from contextual_chips)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GitStatus {
    Clean,
    Modified,
    Untracked,
    Mixed,
}
