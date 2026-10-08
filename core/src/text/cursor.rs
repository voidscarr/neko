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

    pub fn move_left(&mut self, amount: usize, curr_col: usize) {
        if amount > curr_col {
            return;
        }

        self.column = self.column.saturating_sub(amount);
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
            self.sticky_column = self.column;
            return;
        }

        // Move down a row (and try to preserve the current column).
        self.row = self.row.saturating_add(amount);
        self.column = self.sticky_column.min(target_row_len);
    }

    pub fn move_up(&mut self, amount: usize, curr_row: usize, target_row_len: usize) {
        if curr_row == 0 {
            // Move to the start of the current row.
            self.column = 0;
            self.sticky_column = self.column;
            return;
        }

        // Move up a row (and try to preserve the current column).
        self.row = self.row.saturating_sub(amount);
        self.column = self.sticky_column.min(target_row_len);
    }
}
