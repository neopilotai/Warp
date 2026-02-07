use std::cmp::min;

#[derive(Debug, Clone)]
pub struct ClassicEditor {
    content: String,
    cursor_pos: usize,
    history_index: Option<usize>,
}

impl ClassicEditor {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_pos: 0,
            history_index: None,
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        self.content.insert(self.cursor_pos, ch);
        self.cursor_pos += ch.len_utf8();
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            let char_size = self
                .content[..self.cursor_pos]
                .chars()
                .last()
                .map(|c| c.len_utf8())
                .unwrap_or(1);

            self.cursor_pos -= char_size;
            self.content.remove(self.cursor_pos);
        }
    }

    pub fn delete_forward(&mut self) {
        if self.cursor_pos < self.content.len() {
            let next_char_size = self.content[self.cursor_pos..]
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(1);
            
            for _ in 0..next_char_size {
                self.content.remove(self.cursor_pos);
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            let char_size = self.content[..self.cursor_pos]
                .chars()
                .last()
                .map(|c| c.len_utf8())
                .unwrap_or(1);
            self.cursor_pos -= char_size;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.content.len() {
            let char_size = self.content[self.cursor_pos..]
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(1);
            self.cursor_pos += char_size;
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.cursor_pos = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_pos = self.content.len();
    }

    pub fn clear_line(&mut self) {
        self.content.clear();
        self.cursor_pos = 0;
    }

    pub fn current_input(&self) -> &str {
        &self.content
    }

    pub fn set_input(&mut self, input: String) {
        self.content = input;
        self.cursor_pos = self.content.len();
    }

    pub fn clear_input(&mut self) {
        self.content.clear();
        self.cursor_pos = 0;
        self.history_index = None;
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor_pos
    }

    pub fn render_with_cursor(&self) -> String {
        let mut result = String::new();
        for (i, ch) in self.content.chars().enumerate() {
            if i == self.get_cursor_char_index() {
                result.push('|');
            }
            result.push(ch);
        }
        if self.get_cursor_char_index() >= self.content.chars().count() {
            result.push('|');
        }
        result
    }

    fn get_cursor_char_index(&self) -> usize {
        self.content[..self.cursor_pos].chars().count()
    }
}

impl Default for ClassicEditor {
    fn default() -> Self {
        Self::new()
    }
}
