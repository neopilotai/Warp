use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Types of contextual information chips
#[derive(Debug, Clone, PartialEq)]
pub enum ChipType {
    Directory,
    GitStatus,
    Conversation,
    Attachment,
    RuntimeVersion,
    Profile,
    Custom(String),
}

/// A contextual chip providing information about current state
#[derive(Debug, Clone)]
pub struct Chip {
    pub chip_type: ChipType,
    pub label: String,
    pub value: String,
    pub icon: String,
    pub created_at: DateTime<Utc>,
}

impl Chip {
    pub fn new(chip_type: ChipType, label: String, value: String, icon: String) -> Self {
        Self {
            chip_type,
            label,
            value,
            icon,
            created_at: Utc::now(),
        }
    }

    pub fn display(&self) -> String {
        format!("{} {}: {}", self.icon, self.label, self.value)
    }
}

/// Manages contextual chips for display in the input area
#[derive(Debug, Clone)]
pub struct ContextualChips {
    pub chips: Vec<Chip>,
    pub max_chips: usize,
    pub git_info: Option<GitInfo>,
    pub working_directory: String,
    pub active_conversation: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GitInfo {
    pub branch: String,
    pub status: GitStatus,
    pub commit_hash: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GitStatus {
    Clean,
    Modified,
    Untracked,
    Mixed,
}

impl ContextualChips {
    pub fn new() -> Self {
        Self {
            chips: Vec::new(),
            max_chips: 8,
            git_info: None,
            working_directory: "/home/user".to_string(),
            active_conversation: None,
        }
    }

    pub fn add_chip(&mut self, chip: Chip) {
        if self.chips.len() >= self.max_chips {
            self.chips.remove(0); // Remove oldest
        }
        self.chips.push(chip);
    }

    pub fn add_directory_chip(&mut self, path: String) {
        let chip = Chip::new(
            ChipType::Directory,
            "Directory".to_string(),
            path.clone(),
            "📁".to_string(),
        );
        self.working_directory = path;
        self.add_chip(chip);
    }

    pub fn add_git_chip(&mut self, branch: String, status: GitStatus) {
        let status_icon = match status {
            GitStatus::Clean => "✓",
            GitStatus::Modified => "◆",
            GitStatus::Untracked => "◇",
            GitStatus::Mixed => "◈",
        };
        let chip = Chip::new(
            ChipType::GitStatus,
            "Git".to_string(),
            format!("{} ({})", branch, status_icon),
            "⎇".to_string(),
        );
        self.git_info = Some(GitInfo {
            branch,
            status,
            commit_hash: "abc1234".to_string(),
        });
        self.add_chip(chip);
    }

    pub fn add_conversation_chip(&mut self, conversation_id: String) {
        let chip = Chip::new(
            ChipType::Conversation,
            "Conversation".to_string(),
            conversation_id.clone(),
            "💬".to_string(),
        );
        self.active_conversation = Some(conversation_id);
        self.add_chip(chip);
    }

    pub fn add_attachment_chip(&mut self, filename: String) {
        let chip = Chip::new(
            ChipType::Attachment,
            "Attachment".to_string(),
            filename,
            "📎".to_string(),
        );
        self.add_chip(chip);
    }

    pub fn add_runtime_chip(&mut self, runtime: String, version: String) {
        let chip = Chip::new(
            ChipType::RuntimeVersion,
            runtime,
            version,
            "⚙".to_string(),
        );
        self.add_chip(chip);
    }

    pub fn get_display_text(&self) -> String {
        self.chips
            .iter()
            .map(|c| c.display())
            .collect::<Vec<_>>()
            .join("  ")
    }

    pub fn remove_chip_by_type(&mut self, chip_type: ChipType) {
        self.chips.retain(|c| c.chip_type != chip_type);
    }

    pub fn clear_all(&mut self) {
        self.chips.clear();
        self.git_info = None;
        self.active_conversation = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip_creation() {
        let chip = Chip::new(
            ChipType::Directory,
            "Directory".to_string(),
            "/home".to_string(),
            "📁".to_string(),
        );
        assert_eq!(chip.label, "Directory");
        assert_eq!(chip.value, "/home");
    }

    #[test]
    fn test_contextual_chips_add() {
        let mut chips = ContextualChips::new();
        chips.add_directory_chip("/home/user".to_string());
        assert_eq!(chips.chips.len(), 1);
        assert_eq!(chips.working_directory, "/home/user");
    }

    #[test]
    fn test_max_chips_limit() {
        let mut chips = ContextualChips::new();
        chips.max_chips = 3;
        for i in 0..5 {
            chips.add_directory_chip(format!("/path/{}", i));
        }
        assert_eq!(chips.chips.len(), 3);
    }
}
