use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BlockStatus {
    Running,
    Success,
    Failed(String),
    Cancelled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockMetadata {
    pub duration_ms: u64,
    pub timestamp: u64,
    pub directory: String,
    pub git_branch: Option<String>,
    pub bookmarked: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub command: String,
    pub output: BlockOutput,
    pub status: BlockStatus,
    pub metadata: BlockMetadata,
}

impl Block {
    pub fn new(command: String, directory: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            command,
            output: BlockOutput {
                stdout: String::new(),
                stderr: String::new(),
                exit_code: None,
            },
            status: BlockStatus::Running,
            metadata: BlockMetadata {
                duration_ms: 0,
                timestamp,
                directory,
                git_branch: None,
                bookmarked: false,
            },
        }
    }

    pub fn set_output(&mut self, stdout: String, stderr: String, exit_code: i32) {
        self.output = BlockOutput {
            stdout,
            stderr,
            exit_code: Some(exit_code),
        };

        self.status = if exit_code == 0 {
            BlockStatus::Success
        } else {
            BlockStatus::Failed(format!("Exit code: {}", exit_code))
        };
    }

    pub fn get_full_output(&self) -> String {
        let mut result = self.output.stdout.clone();
        if !self.output.stderr.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&self.output.stderr);
        }
        result
    }

    pub fn toggle_bookmark(&mut self) {
        self.metadata.bookmarked = !self.metadata.bookmarked;
    }

    pub fn is_bookmarked(&self) -> bool {
        self.metadata.bookmarked
    }
}
