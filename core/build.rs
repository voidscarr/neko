fn main() {
    cxx_build::bridge("src/ffi.rs").compile("neko");
}
