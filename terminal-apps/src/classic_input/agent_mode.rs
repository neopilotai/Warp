use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Inactive,
    Active,
    Processing,
}

#[derive(Debug, Clone)]
pub struct AgentRequest {
    pub query: String,
    pub context: Option<String>,
    pub requested_permission: bool,
}

#[derive(Debug, Clone)]
pub struct AgentResponse {
    pub suggestion: String,
    pub confidence: f32,
    pub requires_permission: bool,
}

#[derive(Debug, Clone)]
pub struct AgentMode {
    state: AgentState,
    auto_detection_enabled: bool,
    denylist: HashSet<String>,
    natural_language_keywords: Vec<&'static str>,
}

impl AgentMode {
    pub fn new() -> Self {
        Self {
            state: AgentState::Inactive,
            auto_detection_enabled: true,
            denylist: HashSet::new(),
            natural_language_keywords: vec![
                "show", "list", "find", "search", "count", "check", "what", "when", "where",
                "how", "why", "help", "create", "make", "generate", "build", "setup", "fix",
                "debug", "test", "run", "execute", "do", "try", "can you", "please", "would",
            ],
        }
    }

    pub fn toggle(&mut self) {
        self.state = match self.state {
            AgentState::Inactive => AgentState::Active,
            AgentState::Active => AgentState::Inactive,
            AgentState::Processing => AgentState::Inactive,
        };
    }

    pub fn activate(&mut self) {
        self.state = AgentState::Active;
    }

    pub fn deactivate(&mut self) {
        self.state = AgentState::Inactive;
    }

    pub fn set_processing(&mut self) {
        self.state = AgentState::Processing;
    }

    pub fn is_active(&self) -> bool {
        self.state == AgentState::Active || self.state == AgentState::Processing
    }

    pub fn enable_auto_detection(&mut self) {
        self.auto_detection_enabled = true;
    }

    pub fn disable_auto_detection(&mut self) {
        self.auto_detection_enabled = false;
    }

    pub fn add_to_denylist(&mut self, command: String) {
        self.denylist.insert(command);
    }

    pub fn remove_from_denylist(&mut self, command: &str) {
        self.denylist.remove(command);
    }

    pub fn check_natural_language(&mut self, input: &str) {
        if !self.auto_detection_enabled || input.is_empty() {
            return;
        }

        let trimmed = input.trim().to_lowercase();

        // Check denylist
        if self.denylist.iter().any(|cmd| trimmed.starts_with(cmd)) {
            return;
        }

        // Check for natural language keywords
        let has_keyword = self.natural_language_keywords.iter().any(|&kw| {
            trimmed.contains(kw)
        });

        if has_keyword && !self.looks_like_shell_command(input) {
            self.activate();
        }
    }

    fn looks_like_shell_command(&self, input: &str) -> bool {
        let shell_indicators = vec![
            "|", "&&", "||", ">", "<", "$", "`", "(", ")", "{", "}", ";",
            "cd", "ls", "grep", "sed", "awk", "cat", "echo", "mv", "cp",
        ];

        shell_indicators.iter().any(|ind| input.contains(ind))
    }

    pub fn get_state(&self) -> AgentState {
        self.state
    }
}

impl Default for AgentMode {
    fn default() -> Self {
        Self::new()
    }
}
