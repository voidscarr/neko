use crop::Rope;

pub struct Buffer {
    content: Rope,
}

impl Buffer {
    pub fn new_empty() -> Self {
        Self {
            content: Rope::from(""),
        }
    }
}

fn new_empty_buffer() -> Box<Buffer> {
    Box::new(Buffer::new_empty())
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Buffer;

        fn new_empty_buffer() -> Box<Buffer>;
    }
}
