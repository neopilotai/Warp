/// Types of input toolbelt items
#[derive(Debug, Clone, PartialEq)]
pub enum ToolbeltItemType {
    Context,       // @-context mentions
    SlashCommand,  // /slash commands
    VoiceInput,    // Voice-to-text
    Attachments,   // File attachments
    FastForward,   // Quick actions
    Profile,       // User profiles
    ModelPicker,   // AI model selection
    Custom(String),
}

/// A toolbelt item for quick access to input features
#[derive(Debug, Clone)]
pub struct ToolbeltItem {
    pub item_type: ToolbeltItemType,
    pub label: String,
    pub description: String,
    pub hotkey: Option<String>,
    pub icon: String,
}

impl ToolbeltItem {
    pub fn new(
        item_type: ToolbeltItemType,
        label: String,
        description: String,
        icon: String,
    ) -> Self {
        Self {
            item_type,
            label,
            description,
            hotkey: None,
            icon,
        }
    }

    pub fn with_hotkey(mut self, hotkey: String) -> Self {
        self.hotkey = Some(hotkey);
        self
    }

    pub fn display(&self) -> String {
        match &self.hotkey {
            Some(key) => format!("{} {} ({})", self.icon, self.label, key),
            None => format!("{} {}", self.icon, self.label),
        }
    }
}

/// Input toolbelt providing quick access to input features
#[derive(Debug, Clone)]
pub struct InputToolbelt {
    pub items: Vec<ToolbeltItem>,
    pub selected_index: usize,
    pub visible: bool,
}

impl InputToolbelt {
    pub fn new() -> Self {
        let mut toolbelt = Self {
            items: Vec::new(),
            selected_index: 0,
            visible: true,
        };

        // Initialize default items
        toolbelt.add_default_items();
        toolbelt
    }

    fn add_default_items(&mut self) {
        self.add_item(ToolbeltItem::new(
            ToolbeltItemType::Context,
            "Context".to_string(),
            "Add context with @-mentions".to_string(),
            "@".to_string(),
        ).with_hotkey("Ctrl+@".to_string()));

        self.add_item(ToolbeltItem::new(
            ToolbeltItemType::SlashCommand,
            "Commands".to_string(),
            "Quick access to /slash commands".to_string(),
            "/".to_string(),
        ).with_hotkey("Ctrl+/".to_string()));

        self.add_item(ToolbeltItem::new(
            ToolbeltItemType::VoiceInput,
            "Voice".to_string(),
            "Voice-to-text input".to_string(),
            "🎤".to_string(),
        ).with_hotkey("Ctrl+M".to_string()));

        self.add_item(ToolbeltItem::new(
            ToolbeltItemType::Attachments,
            "Attach".to_string(),
            "Attach files or images".to_string(),
            "📎".to_string(),
        ).with_hotkey("Ctrl+K".to_string()));

        self.add_item(ToolbeltItem::new(
            ToolbeltItemType::FastForward,
            "FastForward".to_string(),
            "Quick execution modes".to_string(),
            "⚡".to_string(),
        ).with_hotkey("Ctrl+E".to_string()));

        self.add_item(ToolbeltItem::new(
            ToolbeltItemType::Profile,
            "Profile".to_string(),
            "Switch execution profiles".to_string(),
            "👤".to_string(),
        ).with_hotkey("Ctrl+P".to_string()));

        self.add_item(ToolbeltItem::new(
            ToolbeltItemType::ModelPicker,
            "Models".to_string(),
            "Select AI model for Agent mode".to_string(),
            "🤖".to_string(),
        ).with_hotkey("Ctrl+L".to_string()));
    }

    pub fn add_item(&mut self, item: ToolbeltItem) {
        self.items.push(item);
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

    pub fn select_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected_index = index;
        }
    }

    pub fn get_selected_item(&self) -> Option<&ToolbeltItem> {
        self.items.get(self.selected_index)
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn get_display_items(&self) -> Vec<String> {
        self.items.iter().map(|i| i.display()).collect()
    }

    pub fn find_item_by_type(&self, item_type: &ToolbeltItemType) -> Option<&ToolbeltItem> {
        self.items.iter().find(|i| i.item_type == *item_type)
    }

    pub fn find_item_by_hotkey(&self, hotkey: &str) -> Option<&ToolbeltItem> {
        self.items
            .iter()
            .find(|i| i.hotkey.as_deref() == Some(hotkey))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolbelt_creation() {
        let toolbelt = InputToolbelt::new();
        assert!(toolbelt.items.len() >= 7);
        assert!(toolbelt.visible);
    }

    #[test]
    fn test_toolbelt_navigation() {
        let mut toolbelt = InputToolbelt::new();
        let initial = toolbelt.selected_index;
        toolbelt.select_next();
        assert!(toolbelt.selected_index > initial);

        toolbelt.select_prev();
        assert_eq!(toolbelt.selected_index, initial);
    }

    #[test]
    fn test_find_item_by_hotkey() {
        let toolbelt = InputToolbelt::new();
        let item = toolbelt.find_item_by_hotkey("Ctrl+@");
        assert!(item.is_some());
        assert_eq!(item.unwrap().item_type, ToolbeltItemType::Context);
    }
}
