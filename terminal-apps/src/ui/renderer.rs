use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

/// Terminal renderer using crossterm for raw terminal control
pub struct TerminalRenderer {
    width: u16,
    height: u16,
    buffer: String,
}

impl TerminalRenderer {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let (width, height) = crossterm::terminal::size()?;
        
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        
        Ok(Self {
            width,
            height,
            buffer: String::new(),
        })
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.buffer.push_str("\x1b[2J\x1b[H");
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) {
        self.buffer.push_str(&format!("\x1b[{};{}H", y + 1, x + 1));
    }

    pub fn write(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    pub fn set_color(&mut self, fg: u8, bg: u8) {
        self.buffer.push_str(&format!("\x1b[38;5;{}m\x1b[48;5;{}m", fg, bg));
    }

    pub fn reset_color(&mut self) {
        self.buffer.push_str("\x1b[0m");
    }

    pub fn flush(&mut self) -> io::Result<()> {
        use std::io::Write;
        let mut stdout = io::stdout();
        stdout.write_all(self.buffer.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);
    }
}
