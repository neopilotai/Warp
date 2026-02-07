use std::collections::HashMap;

/// A suggestion for autocomplete or command execution
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub text: String,
    pub description: String,
    pub category: SuggestionCategory,
    pub priority: u8,  // 0-255, higher = more relevant
}

#[derive(Debug, Clone, PartialEq)]
pub enum SuggestionCategory {
    Command,
    File,
    Directory,
    Alias,
    History,
    Custom(String),
}

impl Suggestion {
    pub fn new(
        text: String,
        description: String,
        category: SuggestionCategory,
        priority: u8,
    ) -> Self {
        Self {
            text,
            description,
            category,
            priority,
        }
    }
}

/// Auto-completion engine with context awareness
#[derive(Debug, Clone)]
pub struct AutoCompletion {
    pub available_commands: Vec<String>,
    pub command_descriptions: HashMap<String, String>,
    pub available_files: Vec<String>,
    pub history: Vec<String>,
}

impl AutoCompletion {
    pub fn new() -> Self {
        let mut engine = Self {
            available_commands: vec![
                "ls", "cd", "pwd", "cat", "echo", "grep", "find", "sed",
                "git", "npm", "cargo", "python", "node", "ruby",
            ].iter().map(|s| s.to_string()).collect(),
            command_descriptions: HashMap::new(),
            available_files: Vec::new(),
            history: Vec::new(),
        };

        // Add command descriptions
        engine.command_descriptions.insert("ls".to_string(), "List directory contents".to_string());
        engine.command_descriptions.insert("cd".to_string(), "Change directory".to_string());
        engine.command_descriptions.insert("grep".to_string(), "Search text patterns".to_string());
        engine.command_descriptions.insert("git".to_string(), "Version control system".to_string());

        engine
    }

    pub fn get_suggestions(&self, input: &str) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Command suggestions
        for cmd in &self.available_commands {
            if cmd.starts_with(input) {
                let desc = self.command_descriptions
                    .get(cmd)
                    .cloned()
                    .unwrap_or_default();
                suggestions.push(Suggestion::new(
                    cmd.clone(),
                    desc,
                    SuggestionCategory::Command,
                    100,
                ));
            }
        }

        // File suggestions
        for file in &self.available_files {
            if file.starts_with(input) {
                suggestions.push(Suggestion::new(
                    file.clone(),
                    "File".to_string(),
                    SuggestionCategory::File,
                    50,
                ));
            }
        }

        // History suggestions
        for hist in &self.history {
            if hist.starts_with(input) && !suggestions.iter().any(|s| s.text == *hist) {
                suggestions.push(Suggestion::new(
                    hist.clone(),
                    "From history".to_string(),
                    SuggestionCategory::History,
                    75,
                ));
            }
        }

        // Sort by priority (higher first)
        suggestions.sort_by(|a, b| b.priority.cmp(&a.priority));
        suggestions.truncate(10); // Limit to 10 suggestions

        suggestions
    }

    pub fn add_command(&mut self, cmd: String, description: String) {
        if !self.available_commands.contains(&cmd) {
            self.available_commands.push(cmd.clone());
        }
        self.command_descriptions.insert(cmd, description);
    }

    pub fn add_files(&mut self, files: Vec<String>) {
        self.available_files.extend(files);
    }

    pub fn add_to_history(&mut self, entry: String) {
        if !entry.is_empty() && self.history.last() != Some(&entry) {
            self.history.push(entry);
            if self.history.len() > 1000 {
                self.history.remove(0);
            }
        }
    }
}

/// Error detection and highlighting
#[derive(Debug, Clone)]
pub struct ErrorDetector {
    pub common_typos: HashMap<String, String>,
    pub deprecated_commands: Vec<String>,
}

impl ErrorDetector {
    pub fn new() -> Self {
        let mut typos = HashMap::new();
        typos.insert("gti".to_string(), "git".to_string());
        typos.insert("grpe".to_string(), "grep".to_string());
        typos.insert("lss".to_string(), "ls".to_string());

        Self {
            common_typos: typos,
            deprecated_commands: vec![
                "bzr", "svn", "cvs", "hg",
            ].iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn detect_typo(&self, input: &str) -> Option<String> {
        self.common_typos.get(input).cloned()
    }

    pub fn is_deprecated(&self, command: &str) -> bool {
        self.deprecated_commands.contains(&command.to_string())
    }

    pub fn check_for_errors(&self, input: &str) -> Vec<ParseError> {
        let mut errors = Vec::new();

        // Check for unmatched quotes
        let double_quotes = input.matches('"').count();
        let single_quotes = input.matches('\'').count();

        if double_quotes % 2 != 0 {
            errors.push(ParseError::new(
                "Unmatched double quotes".to_string(),
                ErrorSeverity::Warning,
            ));
        }

        if single_quotes % 2 != 0 {
            errors.push(ParseError::new(
                "Unmatched single quotes".to_string(),
                ErrorSeverity::Warning,
            ));
        }

        // Check for unmatched parentheses
        let open_parens = input.matches('(').count();
        let close_parens = input.matches(')').count();

        if open_parens != close_parens {
            errors.push(ParseError::new(
                "Unmatched parentheses".to_string(),
                ErrorSeverity::Warning,
            ));
        }

        errors
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub severity: ErrorSeverity,
}

impl ParseError {
    pub fn new(message: String, severity: ErrorSeverity) -> Self {
        Self { message, severity }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
}

/// Complete smart features system
#[derive(Debug, Clone)]
pub struct SmartFeatures {
    pub auto_completion: AutoCompletion,
    pub error_detector: ErrorDetector,
    pub enabled: bool,
}

impl SmartFeatures {
    pub fn new() -> Self {
        Self {
            auto_completion: AutoCompletion::new(),
            error_detector: ErrorDetector::new(),
            enabled: true,
        }
    }

    pub fn get_suggestions(&self, input: &str) -> Vec<Suggestion> {
        if self.enabled {
            self.auto_completion.get_suggestions(input)
        } else {
            Vec::new()
        }
    }

    pub fn check_input(&self, input: &str) -> Vec<ParseError> {
        if self.enabled {
            self.error_detector.check_for_errors(input)
        } else {
            Vec::new()
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_completion() {
        let engine = AutoCompletion::new();
        let suggestions = engine.get_suggestions("ls");
        assert!(suggestions.iter().any(|s| s.text == "ls"));
    }

    #[test]
    fn test_error_detection() {
        let detector = ErrorDetector::new();
        let errors = detector.check_for_errors("echo \"hello");
        assert!(errors.iter().any(|e| e.message.contains("Unmatched")));
    }

    #[test]
    fn test_typo_detection() {
        let detector = ErrorDetector::new();
        assert_eq!(detector.detect_typo("gti"), Some("git".to_string()));
    }

    #[test]
    fn test_smart_features() {
        let mut features = SmartFeatures::new();
        assert!(features.enabled);

        let suggestions = features.get_suggestions("ls");
        assert!(!suggestions.is_empty());

        features.toggle();
        assert!(!features.enabled);
    }
}
