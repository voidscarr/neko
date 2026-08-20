use crate::Editor;

fn new_editor() -> Box<Editor> {
    Box::new(Editor::new())
}

#[cxx::bridge]
#[allow(clippy::module_inception)]
mod ffi {

    extern "Rust" {
        type Editor;

        fn new_editor() -> Box<Editor>;

        // Editor
        fn insert_char(self: &mut Editor, index: usize, text: &str);
        fn remove_char(self: &mut Editor, range_start: usize, range_end: usize);
        fn cursor_to_index(self: &Editor) -> usize;
        fn content_slice(self: &Editor, range_start: usize, range_end: usize) -> String;
        fn content_len(self: &Editor) -> usize;
        fn move_left(&mut self, amount: usize);
        fn move_right(&mut self, amount: usize);
    }
}
