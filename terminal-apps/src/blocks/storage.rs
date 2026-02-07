use super::block::Block;
use serde_json;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
pub enum StorageFormat {
    Json,
    Csv,
    PlainText,
}

pub struct BlockStorage;

impl BlockStorage {
    pub fn save_blocks(blocks: &[Block], path: &str, format: StorageFormat) -> Result<(), String> {
        match format {
            StorageFormat::Json => Self::save_as_json(blocks, path),
            StorageFormat::Csv => Self::save_as_csv(blocks, path),
            StorageFormat::PlainText => Self::save_as_plain_text(blocks, path),
        }
    }

    fn save_as_json(blocks: &[Block], path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(blocks)
            .map_err(|e| format!("JSON serialization error: {}", e))?;

        fs::write(path, json).map_err(|e| format!("File write error: {}", e))
    }

    fn save_as_csv(blocks: &[Block], path: &str) -> Result<(), String> {
        let mut csv = String::from("ID,Command,Status,Duration(ms),Directory,Timestamp\n");

        for block in blocks {
            csv.push_str(&format!(
                "\"{}\",\"{}\",\"{:?}\",{},\"{}\",{}\n",
                block.id,
                block.command.replace("\"", "\\\""),
                block.status,
                block.metadata.duration_ms,
                block.metadata.directory,
                block.metadata.timestamp
            ));
        }

        fs::write(path, csv).map_err(|e| format!("File write error: {}", e))
    }

    fn save_as_plain_text(blocks: &[Block], path: &str) -> Result<(), String> {
        let mut text = String::new();

        for block in blocks {
            text.push_str(&format!("$ {}\n", block.command));
            text.push_str(&block.output.stdout);
            if !block.output.stderr.is_empty() {
                text.push_str("\n[stderr]\n");
                text.push_str(&block.output.stderr);
            }
            text.push_str("\n\n---\n\n");
        }

        fs::write(path, text).map_err(|e| format!("File write error: {}", e))
    }

    pub fn load_blocks(path: &str) -> Result<Vec<Block>, String> {
        if !Path::new(path).exists() {
            return Err("File not found".to_string());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| format!("File read error: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("JSON deserialization error: {}", e))
    }
}
