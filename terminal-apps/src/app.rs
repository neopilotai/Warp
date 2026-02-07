use crate::keyset::KeySet;
use crate::theme::Theme;
use std::collections::HashMap;

/// Terminal application state and configuration
pub struct TerminalApp {
    pub name: String,
    pub current_theme: Option<Theme>,
    pub current_keyset: Option<KeySet>,
    pub available_themes: HashMap<String, Theme>,
    pub available_keysets: HashMap<String, KeySet>,
    pub custom_config: HashMap<String, String>,
}

impl TerminalApp {
    pub fn new(name: impl Into<String>) -> Self {
        TerminalApp {
            name: name.into(),
            current_theme: None,
            current_keyset: None,
            available_themes: HashMap::new(),
            available_keysets: HashMap::new(),
            custom_config: HashMap::new(),
        }
    }

    /// Register a theme
    pub fn register_theme(&mut self, theme: Theme) {
        self.available_themes.insert(theme.name.clone(), theme);
    }

    /// Register multiple themes
    pub fn register_themes(&mut self, themes: Vec<Theme>) {
        for theme in themes {
            self.register_theme(theme);
        }
    }

    /// Set the current theme by name
    pub fn set_theme(&mut self, name: &str) -> bool {
        if let Some(theme) = self.available_themes.get(name).cloned() {
            self.current_theme = Some(theme);
            true
        } else {
            false
        }
    }

    /// Register a keyset
    pub fn register_keyset(&mut self, keyset: KeySet) {
        self.available_keysets.insert(keyset.name.clone(), keyset);
    }

    /// Register multiple keysets
    pub fn register_keysets(&mut self, keysets: Vec<KeySet>) {
        for keyset in keysets {
            self.register_keyset(keyset);
        }
    }

    /// Set the current keyset by name
    pub fn set_keyset(&mut self, name: &str) -> bool {
        if let Some(keyset) = self.available_keysets.get(name).cloned() {
            self.current_keyset = Some(keyset);
            true
        } else {
            false
        }
    }

    /// Get a keybinding from the current keyset
    pub fn get_keybinding(&self, action: &str) -> Option<String> {
        self.current_keyset.as_ref().and_then(|ks| ks.get_binding(action).cloned())
    }

    /// Get a color from the current theme
    pub fn get_color(&self, name: &str) -> Option<String> {
        self.current_theme.as_ref().and_then(|t| t.get_color(name))
    }

    /// Set custom configuration
    pub fn set_config(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.custom_config.insert(key.into(), value.into());
    }

    /// Get custom configuration
    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.custom_config.get(key)
    }

    /// List all available themes
    pub fn list_themes(&self) -> Vec<&str> {
        self.available_themes.keys().map(|s| s.as_str()).collect()
    }

    /// List all available keysets
    pub fn list_keysets(&self) -> Vec<&str> {
        self.available_keysets.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = TerminalApp::new("MyApp");
        assert_eq!(app.name, "MyApp");
        assert!(app.current_theme.is_none());
    }

    #[test]
    fn test_register_and_set_theme() {
        let mut app = TerminalApp::new("MyApp");
        let theme = Theme {
            name: "dark".to_string(),
            background: "#000000".to_string(),
            foreground: "#FFFFFF".to_string(),
            accent: "#FF0000".to_string(),
            details: "dark".to_string(),
            terminal_colors: crate::theme::TerminalColors {
                normal: crate::theme::ColorPalette {
                    black: "#000000".to_string(),
                    red: "#FF0000".to_string(),
                    green: "#00FF00".to_string(),
                    yellow: "#FFFF00".to_string(),
                    blue: "#0000FF".to_string(),
                    magenta: "#FF00FF".to_string(),
                    cyan: "#00FFFF".to_string(),
                    white: "#FFFFFF".to_string(),
                },
                bright: crate::theme::ColorPalette {
                    black: "#808080".to_string(),
                    red: "#FF8080".to_string(),
                    green: "#80FF80".to_string(),
                    yellow: "#FFFF80".to_string(),
                    blue: "#8080FF".to_string(),
                    magenta: "#FF80FF".to_string(),
                    cyan: "#80FFFF".to_string(),
                    white: "#FFFFFF".to_string(),
                },
            },
            custom_colors: std::collections::HashMap::new(),
        };

        app.register_theme(theme);
        assert!(app.set_theme("dark"));
        assert!(app.current_theme.is_some());
    }

    #[test]
    fn test_custom_config() {
        let mut app = TerminalApp::new("MyApp");
        app.set_config("debug", "true");
        assert_eq!(app.get_config("debug"), Some(&"true".to_string()));
    }
}
