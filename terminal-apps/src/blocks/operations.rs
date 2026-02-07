use super::block::Block;

#[derive(Clone, Debug)]
pub enum BlockOperation {
    CopyCommand,
    CopyOutput,
    CopyFormattedOutput,
    Bookmark,
    Share(String),
    ReInput,
    Delete,
    ScrollToStart,
}

pub struct BlockOperations;

impl BlockOperations {
    pub fn copy_command(block: &Block) -> String {
        block.command.clone()
    }

    pub fn copy_output(block: &Block) -> String {
        block.output.stdout.clone()
    }

    pub fn copy_formatted_output(block: &Block) -> String {
        let mut result = String::new();
        result.push_str("$ ");
        result.push_str(&block.command);
        result.push('\n');

        if !block.output.stdout.is_empty() {
            result.push_str(&block.output.stdout);
        }

        if !block.output.stderr.is_empty() {
            if !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str("[stderr]\n");
            result.push_str(&block.output.stderr);
        }

        result
    }

    pub fn create_share_link(block: &Block, base_url: &str) -> String {
        let encoded = urlencoding::encode(&block.command);
        format!("{}/blocks?cmd={}&id={}", base_url, encoded, block.id)
    }

    pub fn generate_shell_script(blocks: &[&Block]) -> String {
        let mut script = String::from("#!/bin/bash\n# Generated from Warp Blocks\n\n");

        for block in blocks {
            script.push_str(&format!(
                "# Command: {} (Status: {:?})\n",
                block.command, block.status
            ));
            script.push_str(&format!("{}\n\n", block.command));
        }

        script
    }

    pub fn export_as_json(blocks: &[&Block]) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(blocks)
    }

    pub fn get_command_metadata(block: &Block) -> serde_json::Value {
        serde_json::json!({
            "id": block.id,
            "command": block.command,
            "status": format!("{:?}", block.status),
            "timestamp": block.metadata.timestamp,
            "duration_ms": block.metadata.duration_ms,
            "directory": block.metadata.directory,
            "git_branch": block.metadata.git_branch,
            "exit_code": block.output.exit_code,
            "bookmarked": block.is_bookmarked(),
        })
    }
}
