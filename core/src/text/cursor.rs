use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    row: usize,
    column: usize,
    sticky_column: usize,
}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cursor({0}, {1}, {2})",
            self.row, self.column, self.sticky_column
        )
    }
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

    pub fn at(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
            sticky_column: column,
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

    pub fn move_right(
        &mut self,
        amount: usize,
        max_length: usize,
        curr_row: usize,
        total_rows: usize,
    ) {
        if self.column + amount > max_length && curr_row < total_rows {
            self.column = 0;
            self.row = self.row.saturating_add(1);
            return;
        } else if self.column + amount > max_length {
            return;
        }

        self.column = self.column.saturating_add(amount);
        self.sticky_column = self.column;
    }

    pub fn move_left(
        &mut self,
        amount: usize,
        curr_col: usize,
        curr_row: usize,
        prev_row_len: usize,
    ) {
        if curr_col == 0 && curr_row == 0 {
            return;
        } else if amount > curr_col {
            println!("{amount}");
            println!("curr: {curr_col}");
            self.row = self.row.saturating_sub(amount);
            self.column = prev_row_len;
            self.sticky_column = self.column;
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
        current_row_len: usize,
        target_row_len: usize,
    ) {
        if curr_row >= total_rows {
            // Move to the end of the current row.
            self.column = current_row_len;
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

    pub fn move_to(&mut self, row: usize, column: usize) {
        self.row = row;
        self.column = column;
        self.sticky_column = column;
    }
}
