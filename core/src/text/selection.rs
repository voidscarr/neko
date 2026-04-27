use super::Cursor;

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    start: Cursor,
    end: Cursor,
}
