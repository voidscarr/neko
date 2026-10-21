use super::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    start: Cursor,
    end: Cursor,
    active: bool,
    direction: Direction,
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}

impl Selection {
    pub fn new() -> Self {
        Self {
            start: Cursor::new(),
            end: Cursor::new(),
            active: false,
            direction: Direction::Left,
        }
    }

    pub fn start(&self) -> Cursor {
        self.start
    }

    pub fn end(&self) -> Cursor {
        self.end
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self) {
        self.active = true;
    }

    pub fn clear_active(&mut self) {
        self.active = false;
    }

    pub fn direction(&mut self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }

    // TODO: Consolidate these methods into one?
    pub fn select_left(&mut self, curr_cursor: Cursor, prev_row_len: usize) {
        self.start = curr_cursor;

        if !self.active {
            self.end = curr_cursor;
        }

        self.start
            .move_left(1, self.start.column(), self.start.row(), prev_row_len);
    }

    pub fn select_right(&mut self, curr_cursor: Cursor, max_length: usize, total_rows: usize) {
        self.end = curr_cursor;

        if !self.active {
            self.start = curr_cursor;
        }

        self.end
            .move_right(1, max_length, self.end.row(), total_rows);
    }

    pub fn select_up(&mut self, curr_cursor: Cursor, target_row_len: usize) {
        self.start = curr_cursor;

        if !self.active {
            self.end = curr_cursor;
        }

        self.start.move_up(1, self.start.row(), target_row_len);
    }

    pub fn select_down(
        &mut self,
        curr_cursor: Cursor,
        total_rows: usize,
        current_row_len: usize,
        target_row_len: usize,
    ) {
        self.start = curr_cursor;

        if !self.active {
            self.start = curr_cursor;
        }

        self.end.move_down(
            1,
            self.end.row(),
            total_rows,
            current_row_len,
            target_row_len,
        );
    }

    pub fn select_all(&mut self, rows: usize, last_row_len: usize) {
        self.start = Cursor::new();
        self.end = Cursor::at(rows, last_row_len);
    }
}
