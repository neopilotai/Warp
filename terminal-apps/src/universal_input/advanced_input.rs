use std::collections::HashMap;

/// Input modes: Terminal (shell commands), Agent (AI prompts), Auto (intelligent detection)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Terminal,
    Agent,
    Auto,
}

/// Syntax highlighting rules for different contexts
#[derive(Debug, Clone)]
pub struct SyntaxHighlighting {
    pub keywords: Vec<String>,
    pub operators: Vec<String>,
    pub strings: bool,
    pub comments: bool,
}

impl SyntaxHighlighting {
    pub fn new() -> Self {
        Self {
            keywords: vec![
                "if", "then", "else", "fi", "for", "do", "done", "while",
                "case", "function", "return", "export", "local",
            ].iter().map(|s| s.to_string()).collect(),
            operators: vec![
                "&&", "||", "|", ">", "<", ">>", "<<", "&", ";",
            ].iter().map(|s| s.to_string()).collect(),
            strings: true,
            comments: true,
        }
    }

    pub fn highlight_token(&self, token: &str) -> Option<&str> {
        if self.keywords.contains(&token.to_string()) {
            return Some("keyword");
        }
        if self.operators.contains(&token.to_string()) {
            return Some("operator");
        }
        if token.starts_with('"') && token.ends_with('"') {
            return Some("string");
        }
        if token.starts_with('#') {
            return Some("comment");
        }
        None
    }
}

/// Advanced input component with mode switching and syntax highlighting
#[derive(Debug, Clone)]
pub struct AdvancedInput {
    pub content: String,
    pub mode: InputMode,
    pub cursor_position: usize,
    pub syntax_highlighting: SyntaxHighlighting,
    pub history: Vec<String>,
    pub history_index: Option<usize>,
}

impl AdvancedInput {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            mode: InputMode::Auto,
            cursor_position: 0,
            syntax_highlighting: SyntaxHighlighting::new(),
            history: Vec::new(),
            history_index: None,
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        self.content.insert(self.cursor_position, ch);
        self.cursor_position += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.content.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position < self.content.len() {
            self.content.remove(self.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.content.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.content.len();
    }

    pub fn set_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }

    pub fn add_to_history(&mut self) {
        if !self.content.is_empty() {
            self.history.push(self.content.clone());
        }
    }

    pub fn history_previous(&mut self) {
        if let Some(idx) = self.history_index {
            if idx > 0 {
                self.history_index = Some(idx - 1);
                self.content = self.history[idx - 1].clone();
                self.move_cursor_end();
            }
        } else if !self.history.is_empty() {
            self.history_index = Some(self.history.len() - 1);
            self.content = self.history[self.history.len() - 1].clone();
            self.move_cursor_end();
        }
    }

    pub fn history_next(&mut self) {
        if let Some(idx) = self.history_index {
            if idx < self.history.len() - 1 {
                self.history_index = Some(idx + 1);
                self.content = self.history[idx + 1].clone();
                self.move_cursor_end();
            } else {
                self.history_index = None;
                self.content.clear();
            }
        }
    }

    pub fn clear(&mut self) {
        self.content.clear();
        self.cursor_position = 0;
        self.history_index = None;
    }

    pub fn get_highlighted_lines(&self) -> Vec<Vec<(String, Option<&str>)>> {
        self.content
            .lines()
            .map(|line| {
                let tokens = line.split_whitespace();
                tokens
                    .map(|token| {
                        let highlight_type = self.syntax_highlighting.highlight_token(token);
                        (token.to_string(), highlight_type)
                    })
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_input_creation() {
        let input = AdvancedInput::new();
        assert_eq!(input.content, "");
        assert_eq!(input.mode, InputMode::Auto);
        assert_eq!(input.cursor_position, 0);
    }

    #[test]
    fn test_input_insertion() {
        let mut input = AdvancedInput::new();
        input.insert_char('h');
        input.insert_char('i');
        assert_eq!(input.content, "hi");
        assert_eq!(input.cursor_position, 2);
    }

    #[test]
    fn test_history_navigation() {
        let mut input = AdvancedInput::new();
        input.content = "cmd1".to_string();
        input.add_to_history();
        input.content = "cmd2".to_string();
        input.add_to_history();

        input.history_previous();
        assert_eq!(input.content, "cmd2");

        input.history_previous();
        assert_eq!(input.content, "cmd1");
    }
}
