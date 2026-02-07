/// UI Components matching Warp's design
#[derive(Debug, Clone)]
pub struct Sidebar {
    pub title: String,
    pub items: Vec<SidebarItem>,
    pub selected_index: usize,
}

#[derive(Debug, Clone)]
pub struct SidebarItem {
    pub label: String,
    pub indent: usize,
    pub item_type: SidebarItemType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SidebarItemType {
    Folder,
    Workflow,
    Command,
    Trash,
}

#[derive(Debug, Clone)]
pub struct TabBar {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub title: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct FileList {
    pub items: Vec<FileItem>,
    pub selected_index: usize,
}

#[derive(Debug, Clone)]
pub struct FileItem {
    pub name: String,
    pub size: String,
    pub date: String,
    pub kind: FileKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileKind {
    File,
    Directory,
    Symlink,
}

#[derive(Debug, Clone)]
pub struct CommandBar {
    pub prompt: String,
    pub input: String,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContentPanel {
    pub title: String,
    pub content: String,
    pub scrollable: bool,
    pub scroll_position: usize,
}

impl Sidebar {
    pub fn new(title: String) -> Self {
        Self {
            title,
            items: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn add_item(&mut self, label: String, item_type: SidebarItemType, indent: usize) {
        self.items.push(SidebarItem {
            label,
            indent,
            item_type,
        });
    }

    pub fn select_next(&mut self) {
        if self.selected_index < self.items.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}

impl TabBar {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: 0,
        }
    }

    pub fn add_tab(&mut self, title: String, path: String) {
        self.tabs.push(Tab { title, path });
    }

    pub fn select_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
        }
    }
}

impl FileList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn add_file(&mut self, name: String, size: String, date: String, kind: FileKind) {
        self.items.push(FileItem {
            name,
            size,
            date,
            kind,
        });
    }

    pub fn select_next(&mut self) {
        if self.selected_index < self.items.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}

impl CommandBar {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            input: String::new(),
            suggestions: Vec::new(),
        }
    }

    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
    }
}
