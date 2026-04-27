use super::{Buffer, Cursor, Selection};

/// A single editor instance. Holds relevant document information like the cursor, selection,
/// buffer, etc.
// TODO: Add support for multiple cursors and selections
pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
    selection: Selection,
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::default(),
            cursor: Cursor::default(),
            selection: Selection::default(),
        }
    }

    pub fn insert_char(&mut self, text: &str) {
        self.buffer.insert_char(text);
    }

    pub fn remove_char(&mut self, range_start: usize, range_end: usize) {
        self.buffer.remove_char(range_start, range_end);
    }

    pub fn content_slice(&self, range_start: usize, range_end: usize) -> String {
        self.buffer.content_slice(range_start, range_end)
    }

    pub fn content_len(&self) -> usize {
        self.buffer.len()
    }
}
