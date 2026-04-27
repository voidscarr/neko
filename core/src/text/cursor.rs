#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    row: usize,
    column: usize,
    sticky_column: usize,
}
