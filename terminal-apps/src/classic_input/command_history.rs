use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct CommandHistory {
    commands: VecDeque<String>,
    max_size: usize,
    current_index: Option<usize>,
}

impl CommandHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            commands: VecDeque::with_capacity(max_size),
            max_size,
            current_index: None,
        }
    }

    pub fn add(&mut self, command: String) {
        if !command.trim().is_empty() {
            self.commands.push_back(command);
            if self.commands.len() > self.max_size {
                self.commands.pop_front();
            }
        }
        self.current_index = None;
    }

    pub fn previous(&mut self) -> Option<String> {
        match self.current_index {
            None => {
                if self.commands.is_empty() {
                    None
                } else {
                    self.current_index = Some(self.commands.len() - 1);
                    self.commands.get(self.current_index.unwrap()).cloned()
                }
            }
            Some(idx) if idx > 0 => {
                self.current_index = Some(idx - 1);
                self.commands.get(self.current_index.unwrap()).cloned()
            }
            Some(_) => None,
        }
    }

    pub fn next(&mut self) -> Option<String> {
        match self.current_index {
            None => None,
            Some(idx) if idx < self.commands.len() - 1 => {
                self.current_index = Some(idx + 1);
                self.commands.get(self.current_index.unwrap()).cloned()
            }
            Some(_) => {
                self.current_index = None;
                None
            }
        }
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        self.commands
            .iter()
            .filter(|cmd| cmd.contains(query))
            .cloned()
            .collect()
    }

    pub fn reset_index(&mut self) {
        self.current_index = None;
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn get_all(&self) -> Vec<String> {
        self.commands.iter().cloned().collect()
    }
}
