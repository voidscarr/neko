use super::Cursor;

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    start: Cursor,
    end: Cursor,
    active: bool,
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

    pub fn select_all(&mut self, rows: usize, last_row_len: usize) {
        self.start = Cursor::new();
        self.end = Cursor::at(rows, last_row_len);
    }
}
