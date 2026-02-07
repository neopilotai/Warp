use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Represents a terminal color theme
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Theme {
    pub name: String,
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub details: String,
    pub terminal_colors: TerminalColors,
    #[serde(default)]
    pub custom_colors: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TerminalColors {
    pub normal: ColorPalette,
    pub bright: ColorPalette,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ColorPalette {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

impl Theme {
    pub fn get_color(&self, name: &str) -> Option<String> {
        match name {
            "background" => Some(self.background.clone()),
            "foreground" => Some(self.foreground.clone()),
            "accent" => Some(self.accent.clone()),
            _ => self.custom_colors.get(name).cloned(),
        }
    }
}

/// Theme loading errors
#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("YAML parse error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Theme not found: {0}")]
    NotFound(String),
    #[error("Invalid theme format: {0}")]
    InvalidFormat(String),
}

pub type ThemeResult<T> = Result<T, ThemeError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme {
            name: "test".to_string(),
            background: "#000000".to_string(),
            foreground: "#FFFFFF".to_string(),
            accent: "#FF0000".to_string(),
            details: "dark".to_string(),
            terminal_colors: TerminalColors {
                normal: ColorPalette {
                    black: "#000000".to_string(),
                    red: "#FF0000".to_string(),
                    green: "#00FF00".to_string(),
                    yellow: "#FFFF00".to_string(),
                    blue: "#0000FF".to_string(),
                    magenta: "#FF00FF".to_string(),
                    cyan: "#00FFFF".to_string(),
                    white: "#FFFFFF".to_string(),
                },
                bright: ColorPalette {
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
            custom_colors: HashMap::new(),
        };

        assert_eq!(theme.get_color("background"), Some("#000000".to_string()));
        assert_eq!(theme.get_color("accent"), Some("#FF0000".to_string()));
    }
}
