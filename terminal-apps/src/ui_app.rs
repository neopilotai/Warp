use crate::ui::{ColorScheme, Layout, Rect, TerminalRenderer, UIState};
use std::io;

/// Main Warp-style Terminal UI Application
pub struct WarpTerminalUI {
    renderer: TerminalRenderer,
    state: UIState,
    color_scheme: ColorScheme,
    running: bool,
}

impl WarpTerminalUI {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            renderer: TerminalRenderer::new()?,
            state: UIState::new(),
            color_scheme: ColorScheme::warp(),
            running: true,
        })
    }

    pub fn initialize_demo(&mut self) {
        // Initialize with demo data matching Warp design
        
        // Sidebar items
        self.state.sidebar.add_item("Getting started with Notebooks".to_string(), 
            crate::ui::SidebarItemType::Workflow, 0);
        self.state.sidebar.add_item("Starter workflows".to_string(),
            crate::ui::SidebarItemType::Folder, 0);
        self.state.sidebar.add_item("Example workflow".to_string(),
            crate::ui::SidebarItemType::Workflow, 1);
        self.state.sidebar.add_item("Kill the process running on...".to_string(),
            crate::ui::SidebarItemType::Command, 1);
        self.state.sidebar.add_item("Squash the last N commits".to_string(),
            crate::ui::SidebarItemType::Command, 1);
        self.state.sidebar.add_item("Undo the last Git commit".to_string(),
            crate::ui::SidebarItemType::Command, 1);
        self.state.sidebar.add_item("Trash".to_string(),
            crate::ui::SidebarItemType::Trash, 0);

        // Tabs
        self.state.tab_bar.add_tab("joey@noble: ~/Downloads".to_string(), "~/Downloads".to_string());
        self.state.tab_bar.add_tab("btop".to_string(), "btop".to_string());
        self.state.tab_bar.add_tab("musiccube".to_string(), "musiccube".to_string());

        // File list items
        self.state.file_list.add_file(
            "audacity-linux-3.7.0-x64-22.04.AppImage".to_string(),
            "128M".to_string(),
            "2024-10-30 at 20:27".to_string(),
            crate::ui::FileKind::File,
        );
        self.state.file_list.add_file(
            "big_tux.svg".to_string(),
            "7.3K".to_string(),
            "2024-04-25 at 16:15".to_string(),
            crate::ui::FileKind::File,
        );
        self.state.file_list.add_file(
            "cookie.webp".to_string(),
            "788K".to_string(),
            "2024-04-25 at 16:15".to_string(),
            crate::ui::FileKind::File,
        );
        self.state.file_list.add_file(
            "eg.HEIC".to_string(),
            "12M".to_string(),
            "2024-04-21 at 19:12".to_string(),
            crate::ui::FileKind::File,
        );
        self.state.file_list.add_file(
            "FBReader.2.1.AppImage".to_string(),
            "54M".to_string(),
            "2024-07-23 at 00:01".to_string(),
            crate::ui::FileKind::File,
        );
    }

    pub fn render(&mut self) -> io::Result<()> {
        self.renderer.clear();
        
        let (width, height) = self.renderer.get_size();
        let layout = Layout::new(width, height);
        
        // Split layout
        let (sidebar_rect, content_rect) = layout.split_horizontal(25);
        let (header_rect, main_rect, footer_rect) = 
            layout.split_vertical(header_rect.height + 2, 3);
        
        // Render components
        self.render_sidebar(&sidebar_rect)?;
        self.render_header(&header_rect)?;
        self.render_main(&main_rect)?;
        self.render_footer(&footer_rect)?;
        
        self.renderer.flush()?;
        Ok(())
    }

    fn render_sidebar(&mut self, _rect: &Rect) -> io::Result<()> {
        self.renderer.set_cursor(1, 1);
        self.renderer.write(&format!("┌─ {} ─┐", self.state.sidebar.title));
        
        for (i, item) in self.state.sidebar.items.iter().enumerate() {
            self.renderer.set_cursor(1, (i + 2) as u16);
            
            let prefix = if i == self.state.sidebar.selected_index { "▸" } else { " " };
            let indent = " ".repeat(item.indent * 2);
            let icon = match item.item_type {
                crate::ui::SidebarItemType::Folder => "📁",
                crate::ui::SidebarItemType::Workflow => "⚙️ ",
                crate::ui::SidebarItemType::Command => "$",
                crate::ui::SidebarItemType::Trash => "🗑️ ",
            };
            
            let line = format!("{} {}{} {}", prefix, indent, icon, item.label);
            self.renderer.write(&line);
        }
        
        Ok(())
    }

    fn render_header(&mut self, _rect: &Rect) -> io::Result<()> {
        self.renderer.set_cursor(28, 1);
        
        // Render tabs
        for (i, tab) in self.state.tab_bar.tabs.iter().enumerate() {
            if i == self.state.tab_bar.active_tab {
                self.renderer.write(&format!(" ▶ {} ", tab.title));
            } else {
                self.renderer.write(&format!(" {} ", tab.title));
            }
        }
        
        Ok(())
    }

    fn render_main(&mut self, rect: &Rect) -> io::Result<()> {
        self.renderer.set_cursor(28, 4);
        self.renderer.write("~ /Downloads");
        
        for (i, file) in self.state.file_list.items.iter().enumerate() {
            if i >= (rect.height as usize - 2) {
                break;
            }
            
            self.renderer.set_cursor(28, (6 + i) as u16);
            
            let prefix = if i == self.state.file_list.selected_index { "▸" } else { " " };
            let line = format!("{} {:<30} {:<10} {}", 
                prefix, file.name, file.size, file.date);
            
            self.renderer.write(&line);
        }
        
        Ok(())
    }

    fn render_footer(&mut self, _rect: &Rect) -> io::Result<()> {
        let (_, height) = self.renderer.get_size();
        self.renderer.set_cursor(0, height - 1);
        self.renderer.write(&format!("joey@noble:~/Downloads$ {}", 
            self.state.command_bar.input));
        
        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.initialize_demo();
        self.render()?;
        
        // Event loop would go here
        // For now, just demonstrate rendering
        
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
