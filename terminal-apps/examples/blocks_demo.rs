use warp_terminal_apps::{Block, BlockManager, BlockOperations, BlockRenderer};

fn main() {
    println!("=== Warp Blocks Demo ===\n");

    let mut manager = BlockManager::new(100);

    let mut block1 = Block::new("ls -la".to_string(), "/home/user".to_string());
    block1.set_output(
        "total 48\ndrwxr-xr-x  5 user user 4096 Nov 26 12:34 .\ndrwxr-xr-x 18 root root 4096 Nov 20 10:15 .."
            .to_string(),
        String::new(),
        0,
    );
    block1.metadata.duration_ms = 12;
    manager.add_block(block1.clone());

    let mut block2 = Block::new("git status".to_string(), "/home/user/project".to_string());
    block2.set_output(
        "On branch main\nYour branch is up to date with 'origin/main'.\nnothing to commit, working tree clean"
            .to_string(),
        String::new(),
        0,
    );
    block2.metadata.duration_ms = 145;
    block2.metadata.git_branch = Some("main".to_string());
    manager.add_block(block2.clone());

    let mut block3 = Block::new("cargo build".to_string(), "/home/user/project".to_string());
    block3.set_output(String::new(), "error[E0425]: cannot find value `x` in this scope".to_string(), 101);
    block3.metadata.duration_ms = 2341;
    manager.add_block(block3.clone());

    println!("Blocks in history:");
    for block in manager.get_blocks() {
        println!("{}", BlockRenderer::render_block_compact(block));
    }

    println!("\n=== Detailed Block 1 ===");
    println!("{}", BlockRenderer::render_block(&block1));

    println!("\n=== Block Operations ===");
    println!("Copy Command: {}", BlockOperations::copy_command(&block1));
    println!("Copy Output:\n{}", BlockOperations::copy_output(&block1));
    println!("Formatted:\n{}", BlockOperations::copy_formatted_output(&block1));

    println!("\n=== Bookmarking ===");
    manager.toggle_bookmark(&block1.id).unwrap();
    println!("Bookmarked blocks: {:?}", manager.get_bookmarked().len());

    println!("\n=== Search ===");
    let results = manager.search("git");
    println!("Search for 'git': {} results", results.len());
}
