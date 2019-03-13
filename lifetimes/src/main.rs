use std::cmp::Ordering;

fn main() {
    inner_outer_scope();
    let a = "hello!";
    let b = "world";
    println!("{}", longest(a, b));
}

// Parent and child will have same lifetime
struct Parent<'a>{
    child: &'a i32
}

fn inner_outer_scope() {
    let _a;
    {
        let b = 42;
        _a = &b;
    }
    // println!("{}", _a); // Error: b does not live long enough
}

fn longest<'s>(a: &'s str, b: &'s str) -> &'s str {
    match a.len().cmp(&b.len()) {
        Ordering::Greater => {
            a
        },
        _ => {
            b
        }
    }
}