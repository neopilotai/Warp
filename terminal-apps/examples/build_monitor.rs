use warp_terminal_apps::{ConfigLoader, ExecutionContext, ExtendedWorkflow, KeySet, TerminalApp, Theme, WorkflowStep, Condition};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

/// Build Monitor - Runs workflows and displays real-time progress with colored status
/// 
/// Features:
/// - Execute build pipelines with real-time progress
/// - Conditional workflow steps (if/then/else logic)
/// - Color-coded status indicators
/// - Sequential step execution with error handling
/// - Performance metrics and timing

#[derive(Debug, Clone, Copy)]
enum StepStatus {
    Pending,
    Running,
    Success,
    Failed,
    Skipped,
}

impl StepStatus {
    fn symbol(&self) -> &'static str {
        match self {
            StepStatus::Pending => "⋯",
            StepStatus::Running => "⟳",
            StepStatus::Success => "✓",
            StepStatus::Failed => "✗",
            StepStatus::Skipped => "─",
        }
    }

    fn color_code(&self) -> &'static str {
        match self {
            StepStatus::Pending => "\x1b[37m",    // White
            StepStatus::Running => "\x1b[36m",    // Cyan
            StepStatus::Success => "\x1b[32m",    // Green
            StepStatus::Failed => "\x1b[31m",     // Red
            StepStatus::Skipped => "\x1b[33m",    // Yellow
        }
    }
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

impl BuildMonitor {
    fn new() -> Self {
        let app = TerminalApp::new("Build Monitor");
        BuildMonitor {
            app,
            steps: Vec::new(),
            current_step: 0,
            total_duration_ms: 0,
        }
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create a professional build monitor theme
        let monitor_theme = Theme {
            name: "build_monitor".to_string(),
            background: "#1a1a2e".to_string(),
            foreground: "#eaeaea".to_string(),
            accent: "#00ff88".to_string(),
            details: "dark".to_string(),
            terminal_colors: warp_terminal_apps::theme::TerminalColors {
                normal: warp_terminal_apps::theme::ColorPalette {
                    black: "#1a1a2e".to_string(),
                    red: "#e74c3c".to_string(),
                    green: "#27ae60".to_string(),
                    yellow: "#f39c12".to_string(),
                    blue: "#3498db".to_string(),
                    magenta: "#9b59b6".to_string(),
                    cyan: "#1abc9c".to_string(),
                    white: "#ecf0f1".to_string(),
                },
                bright: warp_terminal_apps::theme::ColorPalette {
                    black: "#34495e".to_string(),
                    red: "#e74c3c".to_string(),
                    green: "#2ecc71".to_string(),
                    yellow: "#f1c40f".to_string(),
                    blue: "#3498db".to_string(),
                    magenta: "#9b59b6".to_string(),
                    cyan: "#1abc9c".to_string(),
                    white: "#ffffff".to_string(),
                },
            },
            custom_colors: HashMap::new(),
        };

        // Minimal keyset for build monitor
        let mut build_keyset = KeySet::new("monitor");
        build_keyset.add_binding("exit", "q");
        build_keyset.add_binding("pause", "space");
        build_keyset.add_binding("view_logs", "l");

        self.app.register_theme(monitor_theme);
        self.app.register_keyset(build_keyset);
        self.app.set_theme("build_monitor");
        self.app.set_keyset("monitor");

        Ok(())
    }

    fn add_step(&mut self, name: String, command: String) {
        self.steps.push(BuildStep {
            name,
            command,
            status: StepStatus::Pending,
            duration_ms: 0,
        });
    }

    fn execute_workflow(&mut self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║            🔨 Build Workflow Execution                 ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        let start_time = std::time::Instant::now();

        for (i, step) in self.steps.iter_mut().enumerate() {
            println!(
                "  {} {} {}",
                step.status.color_code(),
                step.status.symbol(),
                step.name
            );

            // Update to running
            step.status = StepStatus::Running;
            self.display_progress();

            // Simulate work with conditional logic
            let duration = match i {
                0 => 1200,  // cargo check
                1 => 2500,  // cargo build
                2 => 1800,  // cargo test (might be skipped)
                3 => 900,   // cargo clippy
                _ => 500,
            };

            // Simulate work
            thread::sleep(Duration::from_millis(duration / 10));

            // Simulate step success/failure
            let success = match i {
                4 => false,  // Simulate a failure
                _ => true,
            };

            step.duration_ms = duration as u128;

            if success {
                step.status = StepStatus::Success;
            } else {
                step.status = StepStatus::Failed;
                println!(
                    "    {}✗ Build failed at step: {}\x1b[0m",
                    StepStatus::Failed.color_code(),
                    step.name
                );
                break;
            }

            self.display_progress();
        }

        self.total_duration_ms = start_time.elapsed().as_millis();
        self.display_summary();
    }

    fn display_progress(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        for step in &self.steps {
            let duration_str = if step.duration_ms > 0 {
                format!(" ({:>4}ms)", step.duration_ms)
            } else {
                String::new()
            };

            println!(
                "║  {} {} {}{:<40} ║",
                step.status.color_code(),
                step.status.symbol(),
                step.name,
                duration_str
            );
        }
        println!("╚════════════════════════════════════════════════════════╝\x1b[0m");
    }

    fn display_summary(&self) {
        let total_success = self.steps.iter().filter(|s| matches!(s.status, StepStatus::Success)).count();
        let total_failed = self.steps.iter().filter(|s| matches!(s.status, StepStatus::Failed)).count();
        let total_duration_sec = self.total_duration_ms as f64 / 1000.0;

        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║                   BUILD SUMMARY                        ║");
        println!("╠════════════════════════════════════════════════════════╣");
        println!("║  Total Steps: {}                                    ║", self.steps.len());
        println!("║  ✓ Succeeded: {}                                    ║", total_success);
        println!("║  ✗ Failed: {}                                       ║", total_failed);
        println!("║  Total Time: {:.2}s                                ║", total_duration_sec);

        if total_failed == 0 {
            println!("║  Status: SUCCESS ✓                                    ║");
        } else {
            println!("║  Status: FAILED ✗                                     ║");
        }

        println!("╚════════════════════════════════════════════════════════╝");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = BuildMonitor::new();
    monitor.initialize()?;

    // Add build steps
    monitor.add_step("cargo check".to_string(), "Checking code...".to_string());
    monitor.add_step("cargo build".to_string(), "Building project...".to_string());
    monitor.add_step("cargo test".to_string(), "Running tests...".to_string());
    monitor.add_step("cargo clippy".to_string(), "Linting code...".to_string());
    monitor.add_step("cargo doc".to_string(), "Building docs...".to_string());

    monitor.execute_workflow();

    println!("\nPress Enter to continue...");
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input)?;

    Ok(())
}
