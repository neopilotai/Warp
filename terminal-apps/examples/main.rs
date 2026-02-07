use warp_terminal_apps::{ConfigLoader, TerminalApp, Theme, KeySet, ExtendedWorkflow, Condition, WorkflowStep};
use std::path::PathBuf;

/// Example 1: Interactive Task Runner with Theme Support
pub fn example_task_runner() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = TerminalApp::new("TaskRunner");

    // Create a minimal theme programmatically
    let dark_theme = Theme {
        name: "dark".to_string(),
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

    // Create a vim-style keyset
    let mut vim_keyset = KeySet::new("vim");
    vim_keyset.add_binding("quit", "q");
    vim_keyset.add_binding("run_task", "r");
    vim_keyset.add_binding("next_task", "j");
    vim_keyset.add_binding("prev_task", "k");

    // Register theme and keyset
    app.register_theme(dark_theme);
    app.register_keyset(vim_keyset);

    // Set current theme and keyset
    app.set_theme("dark");
    app.set_keyset("vim");

    println!("✓ TaskRunner initialized");
    println!("  Current theme: {:?}", app.current_theme.as_ref().map(|t| &t.name));
    println!("  Current keyset: {:?}", app.current_keyset.as_ref().map(|k| &k.name));
    println!("  Available themes: {:?}", app.list_themes());
    println!("  Available keysets: {:?}", app.list_keysets());

    // Show keybindings
    if let Some(keyset) = &app.current_keyset {
        println!("\nKeybindings:");
        for (action, key) in keyset.list_bindings() {
            println!("  {} -> {}", action, key);
        }
    }

    Ok(())
}

/// Example 2: Workflow Builder with Conditional Execution
pub fn example_workflow_builder() -> Result<(), Box<dyn std::error::Error>> {
    // Create a deployment workflow with conditional steps
    let mut deploy_workflow = ExtendedWorkflow::new("deploy-app");
    deploy_workflow.description = Some("Deploy application with conditional steps".to_string());

    // Set initial variables
    deploy_workflow.set_variable("environment", "staging");
    deploy_workflow.set_variable("version", "1.0.0");

    // Step 1: Run tests
    let test_step = WorkflowStep {
        name: "run_tests".to_string(),
        workflow: "npm:test".to_string(),
        condition: None,
        on_success: Some(vec!["build_app".to_string()]),
        on_failure: Some(vec!["notify_team".to_string()]),
    };

    // Step 2: Build app (conditional on staging)
    let build_step = WorkflowStep {
        name: "build_app".to_string(),
        workflow: "npm:build".to_string(),
        condition: Some(Condition {
            variable: "environment".to_string(),
            operator: "equals".to_string(),
            value: Some("staging".to_string()),
        }),
        on_success: Some(vec!["deploy_staging".to_string()]),
        on_failure: Some(vec!["notify_team".to_string()]),
    };

    // Step 3: Deploy to staging
    let deploy_step = WorkflowStep {
        name: "deploy_staging".to_string(),
        workflow: "docker:push".to_string(),
        condition: None,
        on_success: Some(vec!["run_smoke_tests".to_string()]),
        on_failure: Some(vec!["rollback".to_string()]),
    };

    // Step 4: Run smoke tests
    let smoke_test_step = WorkflowStep {
        name: "run_smoke_tests".to_string(),
        workflow: "test:smoke".to_string(),
        condition: None,
        on_success: None,
        on_failure: Some(vec!["rollback".to_string()]),
    };

    // Step 5: Notify team
    let notify_step = WorkflowStep {
        name: "notify_team".to_string(),
        workflow: "notify:slack".to_string(),
        condition: None,
        on_success: None,
        on_failure: None,
    };

    // Step 6: Rollback
    let rollback_step = WorkflowStep {
        name: "rollback".to_string(),
        workflow: "docker:rollback".to_string(),
        condition: None,
        on_success: Some(vec!["notify_team".to_string()]),
        on_failure: None,
    };

    deploy_workflow.add_step(test_step);
    deploy_workflow.add_step(build_step);
    deploy_workflow.add_step(deploy_step);
    deploy_workflow.add_step(smoke_test_step);
    deploy_workflow.add_step(notify_step);
    deploy_workflow.add_step(rollback_step);

    println!("✓ Deployment workflow created: {}", deploy_workflow.name);
    println!("  Description: {:?}", deploy_workflow.description);
    println!("  Steps: {}", deploy_workflow.steps.len());
    println!("\nWorkflow structure:");

    for (i, step) in deploy_workflow.steps.iter().enumerate() {
        println!("  {}. {} -> {}", i + 1, step.name, step.workflow);
        if let Some(cond) = &step.condition {
            println!(
                "     Condition: {} {} {:?}",
                cond.variable, cond.operator, cond.value
            );
        }
        if let Some(success_steps) = &step.on_success {
            println!("     On success: {:?}", success_steps);
        }
        if let Some(failure_steps) = &step.on_failure {
            println!("     On failure: {:?}", failure_steps);
        }
    }

    Ok(())
}

/// Example 3: Configuration Management
pub fn example_config_management() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = TerminalApp::new("ConfigManager");

    // Set various configurations
    app.set_config("log_level", "debug");
    app.set_config("max_parallel_tasks", "4");
    app.set_config("timeout_seconds", "30");
    app.set_config("enable_notifications", "true");

    println!("✓ Configuration Manager initialized");
    println!("  Configurations:");

    for (key, value) in &app.custom_config {
        println!("    {} = {}", key, value);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Warp Terminal Apps Framework Examples ===\n");

    println!("Example 1: Task Runner with Themes and Keysets");
    println!("---------------------------------------------");
    example_task_runner()?;

    println!("\n\nExample 2: Workflow Builder with Conditional Execution");
    println!("------------------------------------------------------");
    example_workflow_builder()?;

    println!("\n\nExample 3: Configuration Management");
    println!("----------------------------------");
    example_config_management()?;

    println!("\n=== All examples completed successfully ===");

    Ok(())
}
