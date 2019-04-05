use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    name: String,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn print_ct(node: &Rc<Node>) {
    println!(
        "\t{} count:\n\tstrong = {}\n\tweak = {}\n",
        &node.name,
        Rc::strong_count(node),
        Rc::weak_count(node),
    );
}

pub fn run() {
    let leaf = Rc::new(Node {
        name: String::from("leaf"),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("Count after creating leaf:");
    print_ct(&leaf);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    {
        let branch = Rc::new(Node {
            name: String::from("branch"),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        println!("Count after creating branch:");
        print_ct(&branch);

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("Count after adding branch as a parent:");
        print_ct(&branch);
        print_ct(&leaf);
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
    println!("Count after branch goes out of scope:");
    print_ct(&leaf);
}
