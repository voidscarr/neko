use crop::iter::RawLines;

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

    pub fn cursor_to_index(&self) -> usize {
        let mut total = 0;

        for (i, slice) in self.buffer.rows().enumerate() {
            if i == self.cursor.row() {
                return self.cursor.column();
            }

            total += slice.byte_len();
        }

        total += self.cursor.column();
        total
    }

    pub fn insert_char(&mut self, index: usize, text: &str) {
        self.buffer.insert_char(index, text);
        // TODO: Adjust to consider graphemes

        if text.contains("\n") {
            for _ in text.lines() {
                self.cursor.move_down(
                    1,
                    self.cursor.row(),
                    self.buffer.len(),
                    self.buffer.row_len(self.cursor.row().saturating_add(1)),
                );
            }
        }

        self.cursor.move_right(text.len(), self.buffer.len());
    }

    pub fn remove_char(&mut self, range_start: usize, range_end: usize) {
        self.buffer.remove_char(range_start, range_end);
        self.cursor.move_left(range_start, range_end);
    }

    pub fn content_slice(&self, range_start: usize, range_end: usize) -> String {
        self.buffer.content_slice(range_start, range_end)
    }

    pub fn content_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn move_left(&mut self, amount: usize) {
        self.cursor.move_left(0, amount);
    }

    pub fn move_right(&mut self, amount: usize) {
        self.cursor.move_right(amount, self.buffer.len());
    }

    pub fn move_down(&mut self, amount: usize) {
        self.cursor.move_down(
            amount,
            self.cursor.row(),
            self.buffer.rows().len(),
            self.buffer.row_len(self.cursor.row().saturating_add(1)),
        );
    }
}
