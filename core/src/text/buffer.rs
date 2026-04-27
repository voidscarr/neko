use crop::Rope;

pub struct Buffer {
    content: Rope,
}

impl Buffer {
    pub fn new_empty() -> Self {
        Self {
            content: Rope::new(),
        }
    }

    pub fn insert_char(&mut self, text: &str) {
        self.content.insert(self.len(), text);
    }

    pub fn remove_char(&mut self, range_start: usize, range_end: usize) {
        if range_start > range_end || range_end > self.len() {
            return;
        }

        self.content.delete(range_start..range_end);
    }

    pub fn content_slice(&self, range_start: usize, range_end: usize) -> String {
        self.content.byte_slice(range_start..range_end).to_string()
    }

    pub fn len(&self) -> usize {
        self.content.byte_len()
    }
}
