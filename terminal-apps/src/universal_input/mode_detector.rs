use crate::universal_input::advanced_input::InputMode;

/// Detected mode based on input analysis
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DetectedMode {
    Terminal,
    Agent,
    Unknown,
}

impl DetectedMode {
    pub fn to_input_mode(&self) -> InputMode {
        match self {
            DetectedMode::Terminal => InputMode::Terminal,
            DetectedMode::Agent => InputMode::Agent,
            DetectedMode::Unknown => InputMode::Auto,
        }
    }
}

/// Automatically detects whether input is a shell command or AI prompt
#[derive(Debug, Clone)]
pub struct ModeDetector {
    pub terminal_keywords: Vec<String>,
    pub agent_keywords: Vec<String>,
    pub confidence_threshold: f32,
}

impl ModeDetector {
    pub fn new() -> Self {
        Self {
            terminal_keywords: vec![
                // Common shell commands
                "ls", "cd", "pwd", "cat", "echo", "grep", "find", "sed", "awk",
                "cp", "mv", "rm", "mkdir", "rmdir", "chmod", "chown", "tar",
                "git", "npm", "cargo", "python", "node", "ruby", "java",
                // Pipe and redirect operators
                "|", ">", "<", ">>", "&&", "||", ";", "&",
                // Shell builtins
                "if", "then", "else", "fi", "for", "while", "do", "done",
                "function", "return", "export", "alias", "source",
            ].iter().map(|s| s.to_string()).collect(),
            agent_keywords: vec![
                // Natural language indicators
                "help", "explain", "what", "how", "why", "when", "where",
                "generate", "write", "create", "make", "build", "fix", "debug",
                "analyze", "summarize", "describe", "suggest", "recommend",
                "tell", "show", "find", "look", "search", "list",
                // Question indicators
                "?", "please", "can", "could", "would", "should", "might",
            ].iter().map(|s| s.to_string()).collect(),
            confidence_threshold: 0.6,
        }
    }

    /// Analyzes input and returns detected mode
    pub fn detect(&self, input: &str) -> DetectedMode {
        if input.is_empty() {
            return DetectedMode::Unknown;
        }

        let (terminal_score, agent_score) = self.score_input(input);

        if terminal_score > agent_score && terminal_score >= self.confidence_threshold {
            DetectedMode::Terminal
        } else if agent_score > terminal_score && agent_score >= self.confidence_threshold {
            DetectedMode::Agent
        } else {
            DetectedMode::Unknown
        }
    }

    /// Scores input against both terminal and agent patterns
    fn score_input(&self, input: &str) -> (f32, f32) {
        let lower = input.to_lowercase();
        let tokens: Vec<&str> = lower.split_whitespace().collect();

        let mut terminal_matches = 0;
        let mut agent_matches = 0;

        // Check for special shell characters
        if input.contains('|') || input.contains('>') || input.contains('<') 
            || input.contains("&&") || input.contains("||") {
            terminal_matches += 2;
        }

        // Check for quotes (common in shell)
        if input.contains('"') || input.contains('\'') {
            terminal_matches += 1;
        }

        // Count keyword matches
        for token in &tokens {
            if self.terminal_keywords.iter().any(|kw| kw == token) {
                terminal_matches += 1;
            }
            if self.agent_keywords.iter().any(|kw| kw == token) {
                agent_matches += 1;
            }
        }

        // Check for question marks
        if input.ends_with('?') {
            agent_matches += 2;
        }

        // Normalize scores
        let total_tokens = tokens.len().max(1) as f32;
        let terminal_score = terminal_matches as f32 / total_tokens;
        let agent_score = agent_matches as f32 / total_tokens;

        (terminal_score, agent_score)
    }

    /// Get confidence score for a detected mode
    pub fn get_confidence(&self, input: &str) -> f32 {
        let (terminal_score, agent_score) = self.score_input(input);
        terminal_score.max(agent_score)
    }

    /// Get detailed analysis of input
    pub fn analyze(&self, input: &str) -> ModeAnalysis {
        let (terminal_score, agent_score) = self.score_input(input);
        let detected = self.detect(input);
        let confidence = self.get_confidence(input);

        ModeAnalysis {
            detected,
            terminal_score,
            agent_score,
            confidence,
            reasoning: self.get_reasoning(input, detected),
        }
    }

    fn get_reasoning(&self, input: &str, mode: DetectedMode) -> String {
        let lower = input.to_lowercase();

        match mode {
            DetectedMode::Terminal => {
                if input.contains('|') || input.contains('>') {
                    "Detected pipe or redirect operators".to_string()
                } else if lower.split_whitespace().any(|t| self.terminal_keywords.contains(&t.to_string())) {
                    "Detected shell command keywords".to_string()
                } else {
                    "Interpreted as shell command".to_string()
                }
            }
            DetectedMode::Agent => {
                if input.ends_with('?') {
                    "Detected question format".to_string()
                } else if lower.split_whitespace().any(|t| self.agent_keywords.contains(&t.to_string())) {
                    "Detected natural language keywords".to_string()
                } else {
                    "Interpreted as natural language prompt".to_string()
                }
            }
            DetectedMode::Unknown => "Unable to determine with confidence".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModeAnalysis {
    pub detected: DetectedMode,
    pub terminal_score: f32,
    pub agent_score: f32,
    pub confidence: f32,
    pub reasoning: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_detection() {
        let detector = ModeDetector::new();
        assert_eq!(detector.detect("ls -la"), DetectedMode::Terminal);
        assert_eq!(detector.detect("cat file.txt | grep pattern"), DetectedMode::Terminal);
    }

    #[test]
    fn test_agent_detection() {
        let detector = ModeDetector::new();
        assert_eq!(detector.detect("How do I list files?"), DetectedMode::Agent);
        assert_eq!(detector.detect("Explain what grep does"), DetectedMode::Agent);
    }

    #[test]
    fn test_confidence_scoring() {
        let detector = ModeDetector::new();
        let confidence = detector.get_confidence("ls -la");
        assert!(confidence > 0.6);
    }

    #[test]
    fn test_mode_analysis() {
        let detector = ModeDetector::new();
        let analysis = detector.analyze("find . -name *.rs");
        assert_eq!(analysis.detected, DetectedMode::Terminal);
        assert!(analysis.confidence > 0.0);
    }
}
