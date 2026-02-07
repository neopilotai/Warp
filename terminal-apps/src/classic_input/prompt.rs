use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum PromptStyle {
    Warp,
    Custom(String),
    Shell, // Uses PS1
}

#[derive(Debug, Clone)]
pub struct Prompt {
    pub style: PromptStyle,
    pub user: String,
    pub host: String,
    pub current_dir: String,
    pub git_branch: Option<String>,
}

impl Prompt {
    pub fn new(style: PromptStyle) -> Self {
        Self {
            style,
            user: std::env::var("USER").unwrap_or_else(|_| "user".to_string()),
            host: hostname::get()
                .ok()
                .and_then(|h| h.into_string().ok())
                .unwrap_or_else(|| "localhost".to_string()),
            current_dir: std::env::current_dir()
                .ok()
                .and_then(|p| p.to_str().map(|s| s.to_string()))
                .unwrap_or_else(|| "/".to_string()),
            git_branch: None,
        }
    }

    pub fn with_git_branch(mut self, branch: Option<String>) -> Self {
        self.git_branch = branch;
        self
    }

    pub fn render(&self) -> String {
        match &self.style {
            PromptStyle::Warp => self.render_warp_style(),
            PromptStyle::Custom(template) => template.clone(),
            PromptStyle::Shell => self.render_ps1_style(),
        }
    }

    fn render_warp_style(&self) -> String {
        let mut prompt = format!("{}@{} ", self.user, self.host);
        
        // Add directory
        if let Some(home) = std::env::var("HOME").ok() {
            if self.current_dir.starts_with(&home) {
                prompt.push('~');
                prompt.push_str(&self.current_dir[home.len()..]);
            } else {
                prompt.push_str(&self.current_dir);
            }
        } else {
            prompt.push_str(&self.current_dir);
        }

        // Add git branch if present
        if let Some(branch) = &self.git_branch {
            prompt.push_str(" (");
            prompt.push_str(branch);
            prompt.push(')');
        }

        prompt.push_str(" $ ");
        prompt
    }

    fn render_ps1_style(&self) -> String {
        // Render traditional shell prompt
        format!("{}@{}:{}$ ", self.user, self.host, self.current_dir)
    }
}

impl Default for Prompt {
    fn default() -> Self {
        Self::new(PromptStyle::Warp)
    }
}
