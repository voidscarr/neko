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

    pub fn pos_to_index(&self, cursor: &Cursor) -> usize {
        let mut total = 0;

        for (i, slice) in self.buffer.rows().enumerate() {
            if i == cursor.row() {
                total += cursor.column();
                break;
            }

            total += slice.byte_len();
        }

        total
    }

    pub fn cursor_col(self: &Editor) -> usize {
        self.cursor.column()
    }

    pub fn cursor_row(self: &Editor) -> usize {
        self.cursor.row()
    }

    pub fn selection_start(self: &Editor) -> Cursor {
        self.selection.start()
    }

    pub fn selection_end(self: &Editor) -> Cursor {
        self.selection.end()
    }

    pub fn insert_char(&mut self, mut index: usize, text: &str) {
        if self.selection.active() {
            let start = self.pos_to_index(&self.selection.start());
            let end = self.pos_to_index(&self.selection.end());

            self.remove_char(start, end);
            self.selection.clear_active();

            // Adjust index to selection start.
            index = start;
        }

        self.buffer.insert_char(index, text);
        // TODO: Adjust to consider graphemes

        if text.contains("\n") {
            for _ in text.lines() {
                self.cursor.move_down(
                    1,
                    self.cursor.row(),
                    self.buffer.rows().len()
                        - if self.buffer.last_line_has_newline() {
                            0
                        } else {
                            1
                        },
                    self.buffer.row_len(self.cursor.row()),
                    0,
                );
            }
        } else {
            let last_line_has_newline = self.buffer.last_line_has_newline();
            let adjusted_rows_len =
                self.buffer.rows().len() - if last_line_has_newline { 0 } else { 1 };

            self.cursor.move_right(
                text.len(),
                self.buffer.len(),
                self.cursor.row(),
                adjusted_rows_len,
            );
        }
    }

    pub fn remove_char(&mut self, range_start: usize, range_end: usize) {
        let deleted_str = self.buffer.remove_char(range_start, range_end);

        for _ in deleted_str.chars() {
            self.cursor.move_left(
                range_end - range_start,
                self.cursor.column(),
                self.cursor.row(),
                self.buffer.row_len(self.cursor.row().saturating_sub(1)),
            );
        }
    }

    pub fn delete_selection_and_clear(&mut self) {
        if self.selection.active() {
            let start = self.pos_to_index(&self.selection.start());
            let end = self.pos_to_index(&self.selection.end());

            self.remove_char(start, end);
            self.selection.clear_active();
        }
    }

    pub fn content_slice(&self, range_start: usize, range_end: usize) -> String {
        self.buffer.content_slice(range_start, range_end)
    }

    pub fn content_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn selection_active(&self) -> bool {
        self.selection.active()
    }

    pub fn move_left(&mut self, amount: usize) {
        self.cursor.move_left(
            amount,
            self.cursor.column(),
            self.cursor.row(),
            self.buffer.row_len(self.cursor.row().saturating_sub(1)),
        );
        self.selection.clear_active();
    }

    pub fn move_right(&mut self, amount: usize) {
        let last_line_has_newline = self.buffer.last_line_has_newline();
        let adjusted_rows_len =
            self.buffer.rows().len() - if last_line_has_newline { 0 } else { 1 };

        self.cursor.move_right(
            amount,
            self.buffer.row_len(self.cursor.row()),
            self.cursor.row(),
            adjusted_rows_len,
        );
        self.selection.clear_active();
    }

    pub fn move_down(&mut self, amount: usize) {
        let last_line_has_newline = self.buffer.last_line_has_newline();
        let adjusted_rows_len =
            self.buffer.rows().len() - if last_line_has_newline { 0 } else { 1 };
        let adjusted_target_row = if self.cursor.row() == adjusted_rows_len {
            0
        } else {
            1
        };

        self.cursor.move_down(
            amount,
            self.cursor.row(),
            adjusted_rows_len,
            self.buffer.row_len(self.cursor.row()),
            self.buffer
                .row_len(self.cursor.row().saturating_add(adjusted_target_row)),
        );
        self.selection.clear_active();
    }

    pub fn move_up(&mut self, amount: usize) {
        self.cursor.move_up(
            amount,
            self.cursor.row(),
            self.buffer.row_len(self.cursor.row().saturating_sub(1)),
        );
        self.selection.clear_active();
    }

    pub fn select_all(&mut self) {
        let last_line_has_newline = self.buffer.last_line_has_newline();
        let rows = self.buffer.rows().len() - if last_line_has_newline { 0 } else { 1 };
        let last_row_len = self.buffer.row_len(rows);

        self.selection.select_all(rows, last_row_len);
        self.selection.set_active();
        self.cursor.move_to(rows, last_row_len);
    }

    pub fn selected_text(&self) -> String {
        let start = self.pos_to_index(&self.selection.start());
        let end = self.pos_to_index(&self.selection.end());

        self.buffer.content_slice(start, end)
    }

    pub fn num_rows(&self) -> usize {
        self.buffer.rows().len()
    }

    pub fn line(&self, row: usize) -> String {
        self.buffer
            .rows()
            .nth(row)
            .expect("Unable to get line at specified row")
            .to_string()
    }

    pub fn line_len(&self, row: usize) -> usize {
        self.buffer.row_len(row)
    }
}
