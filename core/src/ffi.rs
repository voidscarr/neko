use crop::Rope;

pub struct Buffer {
    content: Rope,
}

impl Buffer {
    pub fn new_empty() -> Self {
        Self {
            content: Rope::new(),
        }
    }

    pub fn insert_char(&mut self, text: &str) {
        self.content.insert(self.len(), text);
    }

    pub fn content(&self, range_start: usize, range_end: usize) -> String {
        self.content.byte_slice(range_start..range_end).to_string()
    }

    pub fn len(&self) -> usize {
        self.content.byte_len()
    }
}

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
        fn content(self: &Buffer, range_start: usize, range_end: usize) -> String;
        fn len(self: &Buffer) -> usize;
    }
}
