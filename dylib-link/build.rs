extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/foo.c")
        .shared_flag(true)
        .pic(true)
        // this is the alias for GCC on my computer
        .compiler("gcc-7")
        .compile("foo.so");
}
