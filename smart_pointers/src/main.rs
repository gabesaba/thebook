mod boxes;
mod deref;
mod drop;
mod rc;

fn main() {
    println!("Boxes:");
    boxes::run();
    println!("\nDeref:");
    deref::run();
    println!("\nDrop:");
    drop::run();
    println!("\nRc:");
    rc::run();
}
