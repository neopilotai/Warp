# Warp Blocks Guide

## Overview

Blocks are atomic units in Warp that group a command and its output together. They enable powerful features like easy copying, sharing, bookmarking, command re-execution, and seamless navigation.

## Core Components

### Block Structure

Each Block contains:
- **Command**: The shell command that was executed
- **Output**: Stdout and stderr streams
- **Status**: Running, Success, Failed, or Cancelled
- **Metadata**: Timestamp, duration, directory, Git branch, bookmark status

```rust
pub struct Block {
    pub id: String,
    pub command: String,
    pub output: BlockOutput,
    pub status: BlockStatus,
    pub metadata: BlockMetadata,
}
```

### Block Manager

The `BlockManager` handles all Block operations:

```rust
let mut manager = BlockManager::new(100); // Max 100 blocks in history
manager.add_block(block);
manager.search("command");
manager.get_bookmarked();
```

## Key Features

### 1. Copy Operations

```rust
// Copy just the command
let cmd = BlockOperations::copy_command(&block);

// Copy just the output
let output = BlockOperations::copy_output(&block);

// Copy formatted (command + output)
let formatted = BlockOperations::copy_formatted_output(&block);
```

### 2. Bookmarking

```rust
block.toggle_bookmark();
let is_bookmarked = block.is_bookmarked();

// Get all bookmarked blocks
let bookmarked = manager.get_bookmarked();
```

### 3. Search

```rust
// Find blocks by command
let results = manager.search("git");
```

### 4. Sharing

```rust
// Generate shareable link
let link = BlockOperations::create_share_link(&block, "https://warp.dev");
```

### 5. Export

```rust
// Export as JSON
let json = BlockOperations::export_as_json(&blocks)?;

// Export as Shell Script
let script = BlockOperations::generate_shell_script(&blocks);
```

### 6. Storage

```rust
// Save blocks to file
BlockStorage::save_blocks(&blocks, "history.json", StorageFormat::Json)?;

// Load blocks from file
let loaded = BlockStorage::load_blocks("history.json")?;
```

## Rendering

Blocks can be rendered in different formats:

```rust
// Full detailed rendering
println!("{}", BlockRenderer::render_block(&block));

// Compact single-line rendering
println!("{}", BlockRenderer::render_block_compact(&block));

// List all blocks
println!("{}", BlockRenderer::render_blocks_list(&blocks));
```

## Integration

### With Terminal UI

```rust
use warp_terminal_apps::{BlockManager, BlockRenderer};

let mut manager = BlockManager::new(1000);
// ... add blocks ...

// Render in terminal
for block in manager.get_blocks() {
    println!("{}", BlockRenderer::render_block_compact(block));
}
```

### With Classic Input

```rust
use warp_terminal_apps::{Block, ClassicInput};

let mut block = Block::new(input_command, current_dir);
// After execution:
block.set_output(stdout, stderr, exit_code);
```

### With Workflows

```rust
// Execute workflow commands, each as a Block
for step in workflow.steps {
    let mut block = Block::new(step.command, current_dir);
    // Execute...
    manager.add_block(block);
}
```

## Example Usage

```rust
use warp_terminal_apps::{Block, BlockManager, BlockRenderer};

fn main() {
    let mut manager = BlockManager::new(100);

    // Create a block
    let mut block = Block::new("ls -la".to_string(), "/home/user".to_string());
    block.set_output("total 48\ndrwxr-xr-x...".to_string(), String::new(), 0);
    block.metadata.duration_ms = 12;

    manager.add_block(block);

    // Render
    for block in manager.get_blocks() {
        println!("{}", BlockRenderer::render_block_compact(block));
    }

    // Bookmark
    manager.toggle_bookmark(&block.id)?;

    // Export
    BlockStorage::save_blocks(&manager.get_blocks(), "history.json", StorageFormat::Json)?;
}
```

## API Reference

### Block

- `new(command, directory)` - Create a new block
- `set_output(stdout, stderr, exit_code)` - Set command output
- `get_full_output()` - Get combined stdout + stderr
- `toggle_bookmark()` - Toggle bookmark status
- `is_bookmarked()` - Check if bookmarked

### BlockManager

- `new(max_history)` - Create new manager
- `add_block(block)` - Add block to history
- `get_blocks()` - Get all blocks
- `get_block(id)` - Get specific block
- `search(query)` - Search blocks by command
- `get_bookmarked()` - Get all bookmarked blocks
- `toggle_bookmark(id)` - Bookmark a block

### BlockOperations

- `copy_command(block)` - Copy command
- `copy_output(block)` - Copy output
- `copy_formatted_output(block)` - Copy formatted
- `create_share_link(block, base_url)` - Generate share link
- `generate_shell_script(blocks)` - Create executable script
- `export_as_json(blocks)` - Export as JSON

### BlockStorage

- `save_blocks(blocks, path, format)` - Save to file
- `load_blocks(path)` - Load from file

### BlockRenderer

- `render_block(block)` - Full rendering with details
- `render_block_compact(block)` - Single-line rendering
- `render_blocks_list(blocks)` - List all blocks
- `render_block_header(block)` - Just the header line
