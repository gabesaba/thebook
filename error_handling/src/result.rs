use std::fs::File;
use std::io::{ErrorKind, self};

pub fn main() {
    open_with_match();
    open_with_unwrap();
    propagate_error();
}

fn open_with_match() {
    let a = File::open("hi.txt");
    let my_file = match a {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => File::create("hi.txt").unwrap(),
            _ => panic!("Failed to open or create file")
        }
    };
}

fn open_with_unwrap() {
    let my_file = File::open("hi.txt").unwrap();
    let my_file = File::open("hi.txt").expect("Unwrap with custom message");
}

fn propagate_error() -> Result<String, io::Error> {
    // Propagate error with question mark
    let f = File::open("hello_world.txt")?;

    // Same as:
    let f = File::open("hello_world.txt");
    let f = match f {
        Ok(t) => t,
        Err(e) => return Err(e)
    };
    Ok(String::from("legit return value"))
}