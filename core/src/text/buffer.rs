use crop::{Rope, iter::RawLines};

pub struct Buffer {
    content: Rope,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            content: Rope::new(),
        }
    }

    pub fn insert_char(&mut self, index: usize, text: &str) {
        self.content.insert(index, text);
    }

    pub fn remove_char(&mut self, range_start: usize, range_end: usize) -> String {
        if range_start > range_end || range_end > self.len() {
            return String::new();
        }

        let deleted_str = self.content_slice(range_start, range_end);
        self.content.delete(range_start..range_end);

        deleted_str
    }

    pub fn content_slice(&self, range_start: usize, range_end: usize) -> String {
        self.content.byte_slice(range_start..range_end).to_string()
    }

    pub fn rows(&self) -> RawLines {
        self.content.raw_lines()
    }

    pub fn row_len(&self, row: usize) -> usize {
        if row == self.rows().len() {
            1
        } else {
            self.content.line_slice(row..row + 1).byte_len()
        }
    }

    pub fn len(&self) -> usize {
        self.content.byte_len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}
