extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/foo.c")
        .shared_flag(true)
        .compiler("gcc-7")
        .compile("foo");
}