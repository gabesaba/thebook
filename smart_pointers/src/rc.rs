enum List {
    Cons(i32, Rc<List>),
    Nil
}

use List::{Cons, Nil};
use std::rc::Rc;

pub fn run() {
    let c = Rc::new(Cons(2, Rc::new(Cons(4, Rc::new(Nil)))));
    println!("Count after creation: {}", Rc::strong_count(&c));

    let _a = Cons(1, Rc::clone(&c));
    println!("Count after cloning for a: {}", Rc::strong_count(&c));
    {
        let _b = Cons(0, Rc::clone(&c));
        println!("Count after cloning for b: {}", Rc::strong_count(&c));
    }
    println!("Count after b goes out of scope: {}", Rc::strong_count(&c));
}
