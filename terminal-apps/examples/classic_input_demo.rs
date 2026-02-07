use warp_terminal_apps::classic_input::{ClassicInput, PromptStyle, Prompt, AgentState};

fn main() {
    let mut input = ClassicInput::new();
    
    println!("=== Warp Classic Input Demo ===\n");
    
    // Demo 1: Rendering with different prompt styles
    println!("1. Prompt Styles:");
    let prompt_warp = Prompt::new(PromptStyle::Warp);
    let prompt_ps1 = Prompt::new(PromptStyle::Shell);
    println!("   Warp Style:  {}", prompt_warp.render());
    println!("   PS1 Style:   {}", prompt_ps1.render());
    println!();
    
    // Demo 2: Text editing operations
    println!("2. Text Editing:");
    input.handle_input('l');
    input.handle_input('s');
    println!("   After typing 'ls': {}", input.editor.current_input());
    input.handle_backspace();
    println!("   After backspace:  {}", input.editor.current_input());
    println!();
    
    // Demo 3: Command history
    println!("3. Command History:");
    let cmd1 = input.submit_command();
    input.handle_input('p');
    input.handle_input('w');
    input.handle_input('d');
    let cmd2 = input.submit_command();
    
    println!("   Command 1: {}", cmd1);
    println!("   Command 2: {}", cmd2);
    
    if let Some(hist) = input.navigate_history_prev() {
        println!("   Previous (via history): {}", hist);
    }
    println!();
    
    // Demo 4: Natural language detection (Agent Mode)
    println!("4. Natural Language Detection (Agent Mode):");
    input.editor.clear_input();
    
    let test_inputs = vec![
        ("list all files in current directory", true),
        ("ls -la", false),
        ("what is the weather", true),
        ("grep -r pattern", false),
        ("find all python files", true),
    ];
    
    for (test_input, expected_agent_mode) in test_inputs {
        input.handle_input('t'); // Reset
        for ch in test_input.chars() {
            input.handle_input(ch);
        }
        
        let is_agent = input.agent_mode.is_active();
        let status = if is_agent == expected_agent_mode { "✓" } else { "✗" };
        println!("   {} '{}' -> Agent Mode: {}", status, test_input, is_agent);
    }
    println!();
    
    // Demo 5: Input hints
    println!("5. Input Hints:");
    println!("   Hints enabled: {}", input.input_hints_enabled);
    if let Some(hint) = input.get_input_hint() {
        println!("   Current hint: {}", hint);
    }
    input.disable_input_hints();
    println!("   Hints disabled: {}", !input.input_hints_enabled);
    println!();
    
    // Demo 6: Text selection modes
    println!("6. Text Selection:");
    input.selection.start_selection(0);
    input.selection.extend_selection(5);
    println!("   Selection active: {}", input.selection.has_active_selection());
    input.selection.toggle_mode();
    println!("   Mode toggled to: {:?}", input.selection.get_selections().first().map(|s| s.mode));
    println!();
    
    println!("=== Demo Complete ===");
}
