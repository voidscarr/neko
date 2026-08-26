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

    pub fn move_right(&mut self, amount: usize) {
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
}
