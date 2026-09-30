#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    row: usize,
    column: usize,
    sticky_column: usize,
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            row: 0,
            column: 0,
            sticky_column: 0,
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn sticky_column(&self) -> usize {
        self.sticky_column
    }

    pub fn move_right(&mut self, amount: usize, max_length: usize) {
        if self.column + amount > max_length {
            return;
        }

        self.column = self.column.saturating_add(amount);
        self.sticky_column = self.column;
    }

    pub fn move_left(&mut self, range_start: usize, range_end: usize) {
        if range_start > range_end {
            return;
        }

        self.column = self.column.saturating_sub(range_end - range_start);
        self.sticky_column = self.column;
    }

    pub fn move_down(
        &mut self,
        amount: usize,
        curr_row: usize,
        total_rows: usize,
        target_row_len: usize,
    ) {
        if curr_row >= total_rows {
            // Move to the end of the current row.
            self.column = target_row_len;
            return;
        }

        // Move down a row (and try to preserve the current column).
        self.row = self.row.saturating_add(amount);
        self.column = self.sticky_column.min(target_row_len);
    }
}
