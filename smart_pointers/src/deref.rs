use std::ops::Deref;

pub fn run() {
    let x = 42;
    let y = MyBox::new(x);

    assert_eq!(42, *y);

    // *y is equivalent to *(y.deref())
    assert_eq!(*y, *(y.deref()));

    let name = MyBox::new(String::from("Rafi"));
    hello(&name);
}

// Deref coercion
fn hello(name: &str) {
    println!("Hello {}", name);
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}




