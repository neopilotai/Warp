use super::{CommandBar, FileList, Sidebar, TabBar};

/// Unified UI state for the entire terminal application
#[derive(Debug, Clone)]
pub struct UIState {
    pub sidebar: Sidebar,
    pub tab_bar: TabBar,
    pub file_list: FileList,
    pub command_bar: CommandBar,
    pub focused_pane: FocusedPane,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusedPane {
    Sidebar,
    TabBar,
    FileList,
    CommandBar,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            sidebar: Sidebar::new("Warp Drive".to_string()),
            tab_bar: TabBar::new(),
            file_list: FileList::new(),
            command_bar: CommandBar::new("❯".to_string()),
            focused_pane: FocusedPane::FileList,
        }
    }

    pub fn focus_pane(&mut self, pane: FocusedPane) {
        self.focused_pane = pane;
    }

    pub fn handle_input(&mut self, key: char) {
        match self.focused_pane {
            FocusedPane::Sidebar => {
                match key {
                    'j' => self.sidebar.select_next(),
                    'k' => self.sidebar.select_prev(),
                    _ => {}
                }
            }
            FocusedPane::FileList => {
                match key {
                    'j' => self.file_list.select_next(),
                    'k' => self.file_list.select_prev(),
                    _ => {}
                }
            }
            FocusedPane::CommandBar => {
                match key {
                    '\x08' => {
                        // Backspace
                        self.command_bar.input.pop();
                    }
                    '\n' => {
                        // Enter
                        self.execute_command();
                    }
                    c if c.is_ascii_graphic() || c == ' ' => {
                        self.command_bar.input.push(c);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn execute_command(&mut self) {
        if !self.command_bar.input.is_empty() {
            // Process command
            self.command_bar.clear_input();
        }
    }

    pub fn reset(&mut self) {
        self.sidebar = Sidebar::new("Warp Drive".to_string());
        self.tab_bar = TabBar::new();
        self.file_list = FileList::new();
        self.command_bar = CommandBar::new("❯".to_string());
        self.focused_pane = FocusedPane::FileList;
    }
}
