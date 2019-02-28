fn main() {
    scope();
    moving();
    cloning();
    types_with_copy();
    let mut name = "ga".to_string();
    add_be(&mut name);
    println!("My name is {}", name);
}

fn add_be(pre: &mut String) {
    pre.push_str("be");
}

fn scope() {
    {
        let person = "gabe".to_string();
        println!("{}", person)
    }
    // Once var leaves scope, heap memory is cleaned up.
    // This is done using the function drop

    // println!("{}", person)
}

fn moving() {
    let person = String::from("jeff");
    let person_2 = person;
    println!("{}", person_2);

    // person_2 now owns the string. Can no longer use person
    // println!("{}", person);
}

fn cloning() {
    let name = String::from("larry");
    let name2 = name.clone();

    println!("{}", name);
    println!("{}", name2);
}

fn types_with_copy() {
    let a: i32 = 5;
    let b = a;
    println!("{} {}", a, b);

    // Simple types that are stored on stack have Copy trait. We can
}
