use crate::{Cursor, Editor};
use ffi::CursorFFI;

fn new_editor() -> Box<Editor> {
    Box::new(Editor::new())
}

#[cxx::bridge]
#[allow(clippy::module_inception)]
mod ffi {
    struct CursorFFI {
        row: usize,
        column: usize,
        sticky_column: usize,
    }

    extern "Rust" {
        type Editor;

        fn new_editor() -> Box<Editor>;

        // Editor
        fn insert_char(self: &mut Editor, index: usize, text: &str);
        fn remove_char(self: &mut Editor, range_start: usize, range_end: usize);
        fn pos_to_index_ffi(editor: &Editor, cursor: &CursorFFI) -> usize;
        fn cursor_col(self: &Editor) -> usize;
        fn cursor_row(self: &Editor) -> usize;
        fn selection_start_ffi(editor: &Box<Editor>) -> CursorFFI;
        fn selection_end_ffi(editor: &Box<Editor>) -> CursorFFI;
        fn selection_active(self: &Editor) -> bool;
        fn delete_selection_and_clear(self: &mut Editor);
        fn content_slice(self: &Editor, range_start: usize, range_end: usize) -> String;
        fn selected_text(self: &Editor) -> String;
        fn line(self: &Editor, row: usize) -> String;
        fn line_len(self: &Editor, row: usize) -> usize;
        fn content_len(self: &Editor) -> usize;
        fn move_left(&mut self, amount: usize);
        fn move_right(&mut self, amount: usize);
        fn move_down(&mut self, amount: usize);
        fn move_up(&mut self, amount: usize);

        fn num_rows(self: &Editor) -> usize;

        fn select_all(&mut self);
    }
}

fn selection_start_ffi(editor: &Editor) -> CursorFFI {
    let start = editor.selection_start();

    CursorFFI {
        row: start.row(),
        column: start.column(),
        sticky_column: start.sticky_column(),
    }
}

fn selection_end_ffi(editor: &Editor) -> CursorFFI {
    let end = editor.selection_end();

    CursorFFI {
        row: end.row(),
        column: end.column(),
        sticky_column: end.sticky_column(),
    }
}

fn pos_to_index_ffi(editor: &Editor, cursor: &CursorFFI) -> usize {
    let mut c = Cursor::new();

    c.move_to(cursor.row, cursor.column);
    editor.pos_to_index(&c)
}
