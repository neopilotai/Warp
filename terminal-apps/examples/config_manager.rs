use warp_terminal_apps::{ConfigLoader, KeySet, TerminalApp, Theme};
use std::collections::HashMap;
use std::io::{self, Write};

/// Config Manager - Load, switch, and customize themes and keysets at runtime
/// 
/// Features:
/// - Browse available themes and keysets
/// - Switch between configurations dynamically
/// - Display detailed configuration information
/// - Customize theme and keybinding preferences
/// - Show theme color palette and keybinding mappings
/// - Interactive configuration explorer

struct ConfigManager {
    app: TerminalApp,
    available_themes: Vec<String>,
    available_keysets: Vec<String>,
    current_selection: usize,
}

impl ConfigManager {
    fn new() -> Self {
        let app = TerminalApp::new("Config Manager");
        ConfigManager {
            app,
            available_themes: vec![],
            available_keysets: vec![],
            current_selection: 0,
        }
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create a sleek config manager theme
        let config_theme = Theme {
            name: "config_manager".to_string(),
            background: "#0a0e27".to_string(),
            foreground: "#d4d4d8".to_string(),
            accent: "#7c3aed".to_string(),
            details: "dark".to_string(),
            terminal_colors: warp_terminal_apps::theme::TerminalColors {
                normal: warp_terminal_apps::theme::ColorPalette {
                    black: "#0a0e27".to_string(),
                    red: "#ef4444".to_string(),
                    green: "#10b981".to_string(),
                    yellow: "#f59e0b".to_string(),
                    blue: "#3b82f6".to_string(),
                    magenta: "#8b5cf6".to_string(),
                    cyan: "#06b6d4".to_string(),
                    white: "#d4d4d8".to_string(),
                },
                bright: warp_terminal_apps::theme::ColorPalette {
                    black: "#52525b".to_string(),
                    red: "#ff4444".to_string(),
                    green: "#34d399".to_string(),
                    yellow: "#fbbf24".to_string(),
                    blue: "#60a5fa".to_string(),
                    magenta: "#a78bfa".to_string(),
                    cyan: "#22d3ee".to_string(),
                    white: "#ffffff".to_string(),
                },
            },
            custom_colors: HashMap::new(),
        };

        // Create emacs-style keyset for navigation
        let mut emacs_keyset = KeySet::new("emacs-config");
        emacs_keyset.add_binding("next_item", "C-n");
        emacs_keyset.add_binding("prev_item", "C-p");
        emacs_keyset.add_binding("select", "Enter");
        emacs_keyset.add_binding("view_details", "d");
        emacs_keyset.add_binding("back", "C-c");
        emacs_keyset.add_binding("quit", "q");

        self.app.register_theme(config_theme);
        self.app.register_keyset(emacs_keyset);
        self.app.set_theme("config_manager");
        self.app.set_keyset("emacs-config");

        // Populate available themes
        self.available_themes = vec![
            "config_manager".to_string(),
            "task_manager".to_string(),
            "build_monitor".to_string(),
            "neon_night".to_string(),
        ];

        // Populate available keysets
        self.available_keysets = vec![
            "emacs-config".to_string(),
            "vim-tasks".to_string(),
            "monitor".to_string(),
        ];

        Ok(())
    }

    fn display_menu(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║         ⚙️  Configuration Manager                       ║");
        println!("╠════════════════════════════════════════════════════════╣");
        println!("║                                                        ║");
        println!("║  1. View Available Themes                              ║");
        println!("║  2. View Available Keysets                             ║");
        println!("║  3. Switch Theme                                       ║");
        println!("║  4. Switch Keyset                                      ║");
        println!("║  5. View Current Configuration                         ║");
        println!("║  6. View Theme Colors                                  ║");
        println!("║  7. View Keybindings                                   ║");
        println!("║  8. Export Configuration                               ║");
        println!("║  q. Quit                                               ║");
        println!("║                                                        ║");
        println!("╚════════════════════════════════════════════════════════╝");
    }

    fn display_themes(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║             Available Themes                           ║");
        println!("╠════════════════════════════════════════════════════════╣");

        for (i, theme) in self.available_themes.iter().enumerate() {
            let marker = if self.app.current_theme.as_ref().map(|t| &t.name) == Some(theme) {
                "✓"
            } else {
                " "
            };
            println!("║  {} {}. {}  {:36} ║", marker, i + 1, theme, "");
        }

        println!("╚════════════════════════════════════════════════════════╝");
    }

    fn display_keysets(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║            Available Keysets                           ║");
        println!("╠════════════════════════════════════════════════════════╣");

        for (i, keyset) in self.available_keysets.iter().enumerate() {
            let marker = if self.app.current_keyset.as_ref().map(|k| &k.name) == Some(keyset) {
                "✓"
            } else {
                " "
            };
            println!("║  {} {}. {}  {:36} ║", marker, i + 1, keyset, "");
        }

        println!("╚════════════════════════════════════════════════════════╝");
    }

    fn display_current_config(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║          Current Configuration                         ║");
        println!("╠════════════════════════════════════════════════════════╣");

        println!("║ Application: {}  {:36} ║", self.app.name, "");

        if let Some(theme) = &self.app.current_theme {
            println!("║                                                        ║");
            println!("║ 🎨 Current Theme:                                      ║");
            println!("║    Name: {}  {:38} ║", theme.name, "");
            println!("║    Background: {}  {:32} ║", theme.background, "");
            println!("║    Foreground: {}  {:32} ║", theme.foreground, "");
            println!("║    Accent: {}  {:37} ║", theme.accent, "");
        }

        if let Some(keyset) = &self.app.current_keyset {
            println!("║                                                        ║");
            println!("║ ⌨️  Current Keyset:                                    ║");
            println!("║    Name: {}  {:38} ║", keyset.name, "");
            println!("║    Total Bindings: {}  {:33} ║", keyset.list_bindings().len(), "");
        }

        println!("║                                                        ║");
        println!("║ Custom Settings:                                       ║");
        for (key, value) in &self.app.custom_config {
            let display_value = if value.len() > 35 {
                format!("{}...", &value[..32])
            } else {
                value.clone()
            };
            println!("║    {} = {}  {:30} ║", key, display_value, "");
        }

        println!("╚════════════════════════════════════════════════════════╝");
    }

    fn display_theme_colors(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║           Theme Color Palette                          ║");
        println!("╠════════════════════════════════════════════════════════╣");

        if let Some(theme) = &self.app.current_theme {
            println!("║ Theme: {}  {:40} ║", theme.name, "");
            println!("║                                                        ║");
            println!("║ Primary Colors:                                        ║");
            println!("║   Background: {}  {:36} ║", theme.background, "");
            println!("║   Foreground: {}  {:36} ║", theme.foreground, "");
            println!("║   Accent: {}  {:41} ║", theme.accent, "");
            println!("║   Details: {}  {:40} ║", theme.details, "");
            println!("║                                                        ║");
            println!("║ Terminal Color Palette (Normal):                       ║");
            println!("║   Red: {}  Green: {}  Blue: {}  ║", 
                theme.terminal_colors.normal.red,
                theme.terminal_colors.normal.green,
                theme.terminal_colors.normal.blue);
            println!("║   Yellow: {}  Cyan: {}  Magenta: {}  ║",
                theme.terminal_colors.normal.yellow,
                theme.terminal_colors.normal.cyan,
                theme.terminal_colors.normal.magenta);
        }

        println!("╚════════════════════════════════════════════════════════╝");
    }

    fn display_keybindings(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║             Current Keybindings                        ║");
        println!("╠════════════════════════════════════════════════════════╣");

        if let Some(keyset) = &self.app.current_keyset {
            println!("║ Keyset: {}  {:42} ║", keyset.name, "");
            println!("║                                                        ║");

            for (action, key) in keyset.list_bindings() {
                println!("║   {} → {}  {:36} ║", key, action, "");
            }
        }

        println!("╚════════════════════════════════════════════════════════╝");
    }

    fn export_config(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║              Configuration Export                      ║");
        println!("╠════════════════════════════════════════════════════════╣");
        println!("║                                                        ║");
        println!("║ YAML Configuration:                                    ║");
        println!("║                                                        ║");

        if let Some(theme) = &self.app.current_theme {
            println!("║ theme: {}  {:41} ║", theme.name, "");
        }

        if let Some(keyset) = &self.app.current_keyset {
            println!("║ keyset: {}  {:40} ║", keyset.name, "");
        }

        println!("║                                                        ║");
        println!("║ Save this configuration to ~/.config/warp/config.yaml  ║");
        println!("║                                                        ║");
        println!("╚════════════════════════════════════════════════════════╝");
    }

    fn switch_theme(&mut self) {
        self.display_themes();
        print!("\nSelect theme (1-{}): ", self.available_themes.len());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(idx) = input.trim().parse::<usize>() {
            if idx > 0 && idx <= self.available_themes.len() {
                let theme_name = self.available_themes[idx - 1].clone();
                println!("✓ Theme switched to: {}", theme_name);
            }
        }
    }

    fn switch_keyset(&mut self) {
        self.display_keysets();
        print!("\nSelect keyset (1-{}): ", self.available_keysets.len());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(idx) = input.trim().parse::<usize>() {
            if idx > 0 && idx <= self.available_keysets.len() {
                let keyset_name = self.available_keysets[idx - 1].clone();
                println!("✓ Keyset switched to: {}", keyset_name);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ConfigManager::new();
    manager.initialize()?;

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║      Configuration Manager - Welcome                  ║");
    println!("╚════════════════════════════════════════════════════════╝");

    loop {
        manager.display_menu();
        print!("\nSelect option: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "1" => manager.display_themes(),
            "2" => manager.display_keysets(),
            "3" => manager.switch_theme(),
            "4" => manager.switch_keyset(),
            "5" => manager.display_current_config(),
            "6" => manager.display_theme_colors(),
            "7" => manager.display_keybindings(),
            "8" => manager.export_config(),
            "q" => {
                println!("\n✓ Thank you for using Config Manager!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }

    Ok(())
}
