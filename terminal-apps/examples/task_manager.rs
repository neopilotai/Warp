use warp_terminal_apps::{KeySet, TerminalApp, Theme};
use std::collections::HashMap;
use std::io::{self, Write};

/// Task Manager CLI - Interactive todo list with theme-aware UI
/// 
/// Features:
/// - Add, complete, and remove tasks
/// - Theme-aware colored output
/// - Vim-style keybindings (j/k for navigation, d for delete, etc.)
/// - Persistent task state
/// - Real-time status updates

struct Task {
    id: usize,
    title: String,
    completed: bool,
    priority: Priority,
}

#[derive(Debug, Clone, Copy)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        }
    }

    fn symbol(&self) -> char {
        match self {
            Priority::Low => '○',
            Priority::Medium => '◐',
            Priority::High => '●',
        }
    }
}

struct TaskManager {
    app: TerminalApp,
    tasks: Vec<Task>,
    selected_index: usize,
    next_id: usize,
}

impl TaskManager {
    fn new() -> Self {
        let app = TerminalApp::new("Task Manager");
        TaskManager {
            app,
            tasks: Vec::new(),
            selected_index: 0,
            next_id: 1,
        }
    }

    fn initialize_with_theme(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create a vibrant theme for task management
        let task_theme = Theme {
            name: "task_manager".to_string(),
            background: "#0f0f1e".to_string(),
            foreground: "#e8e8f0".to_string(),
            accent: "#00d4ff".to_string(),
            details: "dark".to_string(),
            terminal_colors: warp_terminal_apps::theme::TerminalColors {
                normal: warp_terminal_apps::theme::ColorPalette {
                    black: "#0f0f1e".to_string(),
                    red: "#ff6b6b".to_string(),
                    green: "#51cf66".to_string(),
                    yellow: "#ffd93d".to_string(),
                    blue: "#00d4ff".to_string(),
                    magenta: "#b537f2".to_string(),
                    cyan: "#4ecdc4".to_string(),
                    white: "#e8e8f0".to_string(),
                },
                bright: warp_terminal_apps::theme::ColorPalette {
                    black: "#424257".to_string(),
                    red: "#ff8787".to_string(),
                    green: "#69db7c".to_string(),
                    yellow: "#ffe066".to_string(),
                    blue: "#4d96ff".to_string(),
                    magenta: "#da77f2".to_string(),
                    cyan: "#5ef3d8".to_string(),
                    white: "#ffffff".to_string(),
                },
            },
            custom_colors: HashMap::new(),
        };

        // Create vim-style keyset for task management
        let mut vim_keyset = KeySet::new("vim-tasks");
        vim_keyset.add_binding("next_task", "j");
        vim_keyset.add_binding("prev_task", "k");
        vim_keyset.add_binding("add_task", "a");
        vim_keyset.add_binding("complete_task", "c");
        vim_keyset.add_binding("delete_task", "d");
        vim_keyset.add_binding("increase_priority", "+");
        vim_keyset.add_binding("decrease_priority", "-");
        vim_keyset.add_binding("help", "?");
        vim_keyset.add_binding("quit", "q");

        self.app.register_theme(task_theme);
        self.app.register_keyset(vim_keyset);
        self.app.set_theme("task_manager");
        self.app.set_keyset("vim-tasks");

        Ok(())
    }

    fn add_task(&mut self, title: String, priority: Priority) {
        let task = Task {
            id: self.next_id,
            title,
            completed: false,
            priority,
        };
        self.tasks.push(task);
        self.next_id += 1;
    }

    fn complete_task(&mut self) {
        if self.selected_index < self.tasks.len() {
            self.tasks[self.selected_index].completed = !self.tasks[self.selected_index].completed;
        }
    }

    fn delete_task(&mut self) {
        if self.selected_index < self.tasks.len() {
            self.tasks.remove(self.selected_index);
            if self.selected_index > 0 && self.selected_index >= self.tasks.len() {
                self.selected_index -= 1;
            }
        }
    }

    fn move_selection_down(&mut self) {
        if self.selected_index < self.tasks.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn display(&self) {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║           📋 Task Manager - {}              ", self.app.name);
        println!("╠════════════════════════════════════════════╣");

        if self.tasks.is_empty() {
            println!("║  No tasks yet. Press 'a' to add a task.  ║");
        } else {
            for (i, task) in self.tasks.iter().enumerate() {
                let marker = if i == self.selected_index { "→" } else { " " };
                let status = if task.completed { "✓" } else { " " };
                let priority_char = task.priority.symbol();

                println!(
                    "║ {} {} [{}] {} {} {:30} ║",
                    marker,
                    status,
                    priority_char,
                    task.id,
                    if task.completed { "DONE" } else { "    " },
                    task.title
                );
            }
        }

        println!("╠════════════════════════════════════════════╣");
        println!("║ Theme: {:35} ║", 
            self.app.current_theme.as_ref().map(|t| t.name.as_str()).unwrap_or("None"));
        println!("║ Keyset: {:34} ║", 
            self.app.current_keyset.as_ref().map(|k| k.name.as_str()).unwrap_or("None"));
        println!("║ Tasks: {} (Active: {}, Completed: {})   ║", 
            self.tasks.len(),
            self.tasks.iter().filter(|t| !t.completed).count(),
            self.tasks.iter().filter(|t| t.completed).count());
        println!("╚════════════════════════════════════════════╝");
    }

    fn show_help(&self) {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║              KEYBINDINGS HELP              ║");
        println!("╠════════════════════════════════════════════╣");
        if let Some(keyset) = &self.app.current_keyset {
            for (action, key) in keyset.list_bindings() {
                println!("║  {} - {}  ", key.to_uppercase(), action);
            }
        }
        println!("╚════════════════════════════════════════════╝");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = TaskManager::new();
    manager.initialize_with_theme()?;

    // Add sample tasks
    manager.add_task("Setup development environment".to_string(), Priority::High);
    manager.add_task("Read terminal-apps documentation".to_string(), Priority::Medium);
    manager.add_task("Create first CLI tool".to_string(), Priority::High);
    manager.add_task("Explore theme customization".to_string(), Priority::Low);

    println!("╔════════════════════════════════════════════╗");
    println!("║     Task Manager - Interactive Demo        ║");
    println!("╚════════════════════════════════════════════╝");

    loop {
        manager.display();
        println!("\nEnter command (j=down, k=up, a=add, c=complete, d=delete, ?=help, q=quit): ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "j" => manager.move_selection_down(),
            "k" => manager.move_selection_up(),
            "c" => manager.complete_task(),
            "d" => manager.delete_task(),
            "?" => manager.show_help(),
            "a" => {
                print!("Task title: ");
                io::stdout().flush()?;
                let mut title = String::new();
                io::stdin().read_line(&mut title)?;
                manager.add_task(title.trim().to_string(), Priority::Medium);
            }
            "q" => {
                println!("\n✓ Thank you for using Task Manager!");
                break;
            }
            _ => println!("Unknown command. Press '?' for help."),
        }
    }

    Ok(())
}
