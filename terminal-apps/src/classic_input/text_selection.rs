#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Smart,
    Rectangular, // Column selection
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub start: usize,
    pub end: usize,
    pub mode: SelectionMode,
}

#[derive(Debug, Clone)]
pub struct TextSelection {
    selections: Vec<Selection>,
    active: bool,
    mode: SelectionMode,
}

impl TextSelection {
    pub fn new() -> Self {
        Self {
            selections: Vec::new(),
            active: false,
            mode: SelectionMode::Smart,
        }
    }

    pub fn start_selection(&mut self, pos: usize) {
        self.active = true;
        self.selections.clear();
        self.selections.push(Selection {
            start: pos,
            end: pos,
            mode: self.mode,
        });
    }

    pub fn extend_selection(&mut self, pos: usize) {
        if let Some(selection) = self.selections.last_mut() {
            selection.end = pos;
        }
    }

    pub fn end_selection(&mut self) {
        self.active = false;
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            SelectionMode::Smart => SelectionMode::Rectangular,
            SelectionMode::Rectangular => SelectionMode::Smart,
        };
    }

    pub fn get_selected_text(&self, text: &str) -> String {
        self.selections
            .iter()
            .map(|sel| {
                let start = sel.start.min(sel.end);
                let end = sel.start.max(sel.end);
                text.chars()
                    .skip(start)
                    .take(end - start)
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn has_active_selection(&self) -> bool {
        self.active || !self.selections.is_empty()
    }

    pub fn clear(&mut self) {
        self.selections.clear();
        self.active = false;
    }

    pub fn get_selections(&self) -> &[Selection] {
        &self.selections
    }
}

impl Default for TextSelection {
    fn default() -> Self {
        Self::new()
    }
}
