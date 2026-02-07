use super::block::Block;

pub struct BlockRenderer;

impl BlockRenderer {
    pub fn render_block(block: &Block) -> String {
        let mut output = String::new();

        output.push_str(&format!("┌─ Block ID: {} [{}]\n", block.id, block.metadata.timestamp));
        output.push_str(&format!("├─ Command: {}\n", block.command));
        output.push_str(&format!("├─ Status: {:?}\n", block.status));
        output.push_str(&format!("├─ Directory: {}\n", block.metadata.directory));

        if let Some(branch) = &block.metadata.git_branch {
            output.push_str(&format!("├─ Branch: {}\n", branch));
        }

        output.push_str(&format!("├─ Duration: {}ms\n", block.metadata.duration_ms));

        if block.is_bookmarked() {
            output.push_str("├─ [★ Bookmarked]\n");
        }

        output.push_str("├─ Output:\n");
        for line in block.output.stdout.lines() {
            output.push_str(&format!("│  {}\n", line));
        }

        if !block.output.stderr.is_empty() {
            output.push_str("├─ Stderr:\n");
            for line in block.output.stderr.lines() {
                output.push_str(&format!("│  [ERR] {}\n", line));
            }
        }

        output.push_str("└─ End Block\n");
        output
    }

    pub fn render_block_compact(block: &Block) -> String {
        let status_icon = match &block.status {
            super::block::BlockStatus::Success => "✓",
            super::block::BlockStatus::Failed(_) => "✗",
            super::block::BlockStatus::Running => "⟳",
            super::block::BlockStatus::Cancelled => "◯",
        };

        let bookmark = if block.is_bookmarked() { " ★" } else { "" };

        format!(
            "{} [{}] {} {}{}",
            status_icon, block.metadata.timestamp, block.command, block.metadata.duration_ms, bookmark
        )
    }

    pub fn render_blocks_list(blocks: &[&Block]) -> String {
        let mut output = String::new();
        output.push_str("Blocks History:\n");

        for (idx, block) in blocks.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", idx + 1, Self::render_block_compact(block)));
        }

        output
    }

    pub fn render_block_header(block: &Block) -> String {
        format!(
            "$ {} [{}] {:?}",
            block.command, block.metadata.duration_ms, block.status
        )
    }
}
