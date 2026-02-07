use warp_terminal_apps::WarpTerminalUI;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut ui = WarpTerminalUI::new()?;
    ui.run()?;
    
    println!("\nWarp Terminal UI initialized successfully!");
    println!("Press Enter to exit...");
    
    let mut input = [0; 1];
    let _ = io::stdin().read(&mut input);
    
    Ok(())
}
