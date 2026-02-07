use warp_terminal_apps::{ConfigLoader, ExecutionContext, ExtendedWorkflow, TerminalApp};

/// Example: Interactive CLI Tool Builder
///
/// This example demonstrates how to build a complete interactive terminal
/// application using themes, keysets, and workflows.

struct InteractiveApp {
    app: TerminalApp,
    workflows: Vec<ExtendedWorkflow>,
    current_workflow_index: usize,
}

impl InteractiveApp {
    fn new(name: &str) -> Self {
        InteractiveApp {
            app: TerminalApp::new(name),
            workflows: Vec::new(),
            current_workflow_index: 0,
        }
    }

    fn initialize_with_defaults(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create default theme
        let default_theme = warp_terminal_apps::Theme {
            name: "default".to_string(),
            background: "#1e1e1e".to_string(),
            foreground: "#e0e0e0".to_string(),
            accent: "#007acc".to_string(),
            details: "dark".to_string(),
            terminal_colors: warp_terminal_apps::theme::TerminalColors {
                normal: warp_terminal_apps::theme::ColorPalette {
                    black: "#1e1e1e".to_string(),
                    red: "#f48771".to_string(),
                    green: "#6a9955".to_string(),
                    yellow: "#dcdcaa".to_string(),
                    blue: "#569cd6".to_string(),
                    magenta: "#c586c0".to_string(),
                    cyan: "#4ec9b0".to_string(),
                    white: "#e0e0e0".to_string(),
                },
                bright: warp_terminal_apps::theme::ColorPalette {
                    black: "#666666".to_string(),
                    red: "#f48771".to_string(),
                    green: "#6a9955".to_string(),
                    yellow: "#dcdcaa".to_string(),
                    blue: "#569cd6".to_string(),
                    magenta: "#c586c0".to_string(),
                    cyan: "#4ec9b0".to_string(),
                    white: "#ffffff".to_string(),
                },
            },
            custom_colors: std::collections::HashMap::new(),
        };

        // Create default keyset
        let mut default_keyset = warp_terminal_apps::KeySet::new("default");
        default_keyset.add_binding("app:quit", "q");
        default_keyset.add_binding("app:next_item", "j");
        default_keyset.add_binding("app:prev_item", "k");
        default_keyset.add_binding("app:select", "enter");
        default_keyset.add_binding("app:help", "?");

        self.app.register_theme(default_theme);
        self.app.register_keyset(default_keyset);

        self.app.set_theme("default");
        self.app.set_keyset("default");

        // Set default configuration
        self.app.set_config("debug_mode", "false");
        self.app.set_config("animation_enabled", "true");
        self.app.set_config("log_level", "info");

        Ok(())
    }

    fn add_workflow(&mut self, workflow: ExtendedWorkflow) {
        self.workflows.push(workflow);
    }

    fn display_status(&self) {
        println!("\n╔════════════════════════════════════════╗");
        println!("║ {} Terminal Application", self.app.name);
        println!("╠════════════════════════════════════════╣");
        println!("║ Theme: {:28} ║", {
            self.app
                .current_theme
                .as_ref()
                .map(|t| &t.name)
                .unwrap_or(&"None".to_string())
        });
        println!("║ Keyset: {:27} ║", {
            self.app
                .current_keyset
                .as_ref()
                .map(|k| &k.name)
                .unwrap_or(&"None".to_string())
        });
        println!("║ Active Workflows: {:19} ║", self.workflows.len());
        println!("╚════════════════════════════════════════╝");
    }

    fn display_keybindings(&self) {
        println!("\n📌 Available Keybindings:");
        println!("─────────────────────────");

        if let Some(keyset) = &self.app.current_keyset {
            for (action, key) in keyset.list_bindings() {
                println!("  {} → {}", key.to_uppercase(), action);
            }
        }
    }

    fn display_themes(&self) {
        println!("\n🎨 Available Themes:");
        println!("────────────────────");

        for theme_name in self.app.list_themes() {
            let marker = if Some(theme_name) == self.app.current_theme.as_ref().map(|t| t.name.as_str()) {
                "✓"
            } else {
                " "
            };
            println!("  {} {}", marker, theme_name);
        }
    }

    fn display_workflows(&self) {
        println!("\n🔄 Registered Workflows:");
        println!("─────────────────────────");

        for (i, workflow) in self.workflows.iter().enumerate() {
            println!("  {}. {}", i + 1, workflow.name);
            if let Some(desc) = &workflow.description {
                println!("     └─ {}", desc);
            }
            println!("     Steps: {}", workflow.steps.len());
        }
    }

    fn display_configuration(&self) {
        println!("\n⚙️  Application Configuration:");
        println!("────────────────────────────");

        for (key, value) in &self.app.custom_config {
            println!("  {} = {}", key, value);
        }
    }

    fn run_workflow(&self, index: usize) -> Result<(), String> {
        if index >= self.workflows.len() {
            return Err(format!("Workflow {} not found", index));
        }

        let workflow = &self.workflows[index];
        println!("\n▶️  Running workflow: {}", workflow.name);

        if let Some(desc) = &workflow.description {
            println!("   Description: {}", desc);
        }

        println!("   Steps:");
        for (i, step) in workflow.steps.iter().enumerate() {
            println!("   {}. {} → {}", i + 1, step.name, step.workflow);

            if let Some(condition) = &step.condition {
                println!("      [Condition: {} {} {:?}]", condition.variable, condition.operator, condition.value);
            }
        }

        println!("   ✓ Workflow structure validated");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║  Warp Terminal Apps - Interactive CLI Tool Example     ║");
    println!("╚════════════════════════════════════════════════════════╝");

    // Create and initialize the app
    let mut cli_app = InteractiveApp::new("WarpCLI");
    cli_app.initialize_with_defaults()?;

    // Add some example workflows
    let mut build_workflow = ExtendedWorkflow::new("build");
    build_workflow.description = Some("Build the application".to_string());
    build_workflow.set_variable("compiler", "rustc");

    let mut test_workflow = ExtendedWorkflow::new("test");
    test_workflow.description = Some("Run all tests".to_string());
    test_workflow.set_variable("test_framework", "cargo test");

    let mut deploy_workflow = ExtendedWorkflow::new("deploy");
    deploy_workflow.description = Some("Deploy to production".to_string());
    deploy_workflow.set_variable("environment", "production");

    cli_app.add_workflow(build_workflow);
    cli_app.add_workflow(test_workflow);
    cli_app.add_workflow(deploy_workflow);

    // Display application status
    cli_app.display_status();

    // Display available themes
    cli_app.display_themes();

    // Display available keybindings
    cli_app.display_keybindings();

    // Display available workflows
    cli_app.display_workflows();

    // Display configuration
    cli_app.display_configuration();

    // Demonstrate workflow execution
    println!("\n▼ Demonstrating workflow execution:");
    cli_app.run_workflow(0)?;

    println!("\n✓ Interactive CLI tool example completed successfully!");

    Ok(())
}
