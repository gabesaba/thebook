fn main() {
    println!("Hello, world!");
    // something_bad_happened()
    // invalid_access()
}

fn something_bad_happened() {
    // Panicking will unwind stack. Can just have program abort, however (see Cargo.toml)
    panic!("ahh help it hurts")
}

fn invalid_access() {
    let a = vec![0,2,3,4];
    println!("{}", a[4]);
}