/// Layout system for Warp terminal UI
#[derive(Debug, Clone)]
pub struct Layout {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Layout {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Split screen into sidebar + main content
    pub fn split_horizontal(&self, sidebar_width: u16) -> (Rect, Rect) {
        let sidebar = Rect {
            x: 0,
            y: 0,
            width: sidebar_width,
            height: self.height,
        };
        let content = Rect {
            x: sidebar_width,
            y: 0,
            width: self.width - sidebar_width,
            height: self.height,
        };
        (sidebar, content)
    }

    /// Split content area into header + main + footer
    pub fn split_vertical(&self, header_height: u16, footer_height: u16) -> (Rect, Rect, Rect) {
        let header = Rect {
            x: 0,
            y: 0,
            width: self.width,
            height: header_height,
        };
        let main = Rect {
            x: 0,
            y: header_height,
            width: self.width,
            height: self.height - header_height - footer_height,
        };
        let footer = Rect {
            x: 0,
            y: self.height - footer_height,
            width: self.width,
            height: footer_height,
        };
        (header, main, footer)
    }

    /// Create a bordered rectangle
    pub fn create_border(&self, rect: &Rect) -> String {
        let mut border = String::new();
        
        // Top border
        border.push('┌');
        border.push_str(&"─".repeat((rect.width.saturating_sub(2)) as usize));
        border.push('┐');
        
        border
    }
}
