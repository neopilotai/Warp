pub mod agent_mode;
pub mod command_history;
pub mod editor;
pub mod prompt;
pub mod text_selection;

pub use agent_mode::{AgentMode, AgentRequest, AgentResponse, AgentState};
pub use command_history::CommandHistory;
pub use editor::ClassicEditor;
pub use prompt::Prompt;
pub use text_selection::{Selection, SelectionMode, TextSelection};

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ClassicInput {
    pub editor: ClassicEditor,
    pub prompt: Prompt,
    pub history: CommandHistory,
    pub agent_mode: AgentMode,
    pub selection: TextSelection,
    pub input_hints_enabled: bool,
}

impl ClassicInput {
    pub fn new() -> Self {
        Self {
            editor: ClassicEditor::new(),
            prompt: Prompt::default(),
            history: CommandHistory::new(1000),
            agent_mode: AgentMode::new(),
            selection: TextSelection::new(),
            input_hints_enabled: true,
        }
    }

    pub fn enable_input_hints(&mut self) {
        self.input_hints_enabled = true;
    }

    pub fn disable_input_hints(&mut self) {
        self.input_hints_enabled = false;
    }

    pub fn get_input_hint(&self) -> Option<&'static str> {
        if !self.input_hints_enabled {
            return None;
        }

        match self.editor.current_input().len() {
            0 => Some("Type a command or natural language query..."),
            _ if self.agent_mode.is_active() => Some("Press ENTER to send query to AI, or ESC to cancel"),
            _ => None,
        }
    }

    pub fn toggle_agent_mode(&mut self) {
        self.agent_mode.toggle();
    }

    pub fn handle_input(&mut self, ch: char) {
        self.editor.insert_char(ch);
        self.agent_mode.check_natural_language(self.editor.current_input());
    }

    pub fn handle_backspace(&mut self) {
        self.editor.backspace();
    }

    pub fn navigate_history_prev(&mut self) {
        if let Some(cmd) = self.history.previous() {
            self.editor.set_input(cmd);
        }
    }

    pub fn navigate_history_next(&mut self) {
        if let Some(cmd) = self.history.next() {
            self.editor.set_input(cmd);
        } else {
            self.editor.clear_input();
        }
    }

    pub fn submit_command(&mut self) -> String {
        let cmd = self.editor.current_input().to_string();
        self.history.add(cmd.clone());
        self.editor.clear_input();
        cmd
    }

    pub fn render_input_line(&self) -> String {
        format!("{}{}", self.prompt.render(), self.editor.current_input())
    }
}

impl Default for ClassicInput {
    fn default() -> Self {
        Self::new()
    }
}
