use super::Cursor;

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    start: Cursor,
    end: Cursor,
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
        }
    }
}
