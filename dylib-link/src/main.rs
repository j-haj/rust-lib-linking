
extern {
    fn hello();
}

fn main() {
    println!("Calling function from dylib...");
    unsafe { hello() }
    println!("Done");
}
