use crate::Buffer;

fn new_empty_buffer() -> Box<Buffer> {
    Box::new(Buffer::new_empty())
}

#[cxx::bridge]
#[allow(clippy::module_inception)]
mod ffi {
    extern "Rust" {
        type Buffer;

        fn new_empty_buffer() -> Box<Buffer>;

        // Buffer
        fn insert_char(self: &mut Buffer, text: &str);
        fn remove_char(self: &mut Buffer, range_start: usize, range_end: usize);
        fn content_slice(self: &Buffer, range_start: usize, range_end: usize) -> String;
        fn len(self: &Buffer) -> usize;
    }
}
