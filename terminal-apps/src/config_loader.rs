use crate::keyset::{KeySet, KeySetError, KeySetResult};
use crate::theme::{Theme, ThemeError, ThemeResult};
use std::fs;
use std::path::Path;

/// Configuration loader for themes and keysets
pub struct ConfigLoader;

impl ConfigLoader {
    /// Load a theme from a YAML file
    pub fn load_theme<P: AsRef<Path>>(path: P) -> ThemeResult<Theme> {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str(&content).map_err(|e| ThemeError::YamlError(e))
    }

    /// Load a keyset from a YAML file
    pub fn load_keyset<P: AsRef<Path>>(path: P) -> KeySetResult<KeySet> {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str(&content).map_err(|e| KeySetError::YamlError(e))
    }

    /// Load all themes from a directory
    pub fn load_themes_from_directory<P: AsRef<Path>>(dir: P) -> ThemeResult<Vec<Theme>> {
        let mut themes = Vec::new();
        let entries = fs::read_dir(dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml")
                || path.extension().and_then(|s| s.to_str()) == Some("yml")
            {
                if let Ok(theme) = Self::load_theme(&path) {
                    themes.push(theme);
                }
            }
        }

        Ok(themes)
    }

    /// Load all keysets from a directory
    pub fn load_keysets_from_directory<P: AsRef<Path>>(dir: P) -> KeySetResult<Vec<KeySet>> {
        let mut keysets = Vec::new();
        let entries = fs::read_dir(dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml")
                || path.extension().and_then(|s| s.to_str()) == Some("yml")
            {
                if let Ok(keyset) = Self::load_keyset(&path) {
                    keysets.push(keyset);
                }
            }
        }

        Ok(keysets)
    }

    /// Save a theme to a YAML file
    pub fn save_theme<P: AsRef<Path>>(theme: &Theme, path: P) -> ThemeResult<()> {
        let yaml = serde_yaml::to_string(theme)?;
        fs::write(path, yaml)?;
        Ok(())
    }

    /// Save a keyset to a YAML file
    pub fn save_keyset<P: AsRef<Path>>(keyset: &KeySet, path: P) -> KeySetResult<()> {
        let yaml = serde_yaml::to_string(keyset)?;
        fs::write(path, yaml)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_save_and_load_theme() {
        let temp_dir = std::env::temp_dir();
        let theme_path = temp_dir.join("test_theme.yaml");

        let theme = Theme {
            name: "test".to_string(),
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

        ConfigLoader::save_theme(&theme, &theme_path).unwrap();
        let loaded = ConfigLoader::load_theme(&theme_path).unwrap();

        assert_eq!(theme.name, loaded.name);
        assert_eq!(theme.background, loaded.background);

        let _ = fs::remove_file(theme_path);
    }
}
