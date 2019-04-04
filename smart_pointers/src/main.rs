mod boxes;
mod deref;
mod drop;
mod rc;
mod ref_cell;

fn main() {
    println!("Boxes:");
    boxes::run();
    println!("\nDeref:");
    deref::run();
    println!("\nDrop:");
    drop::run();
    println!("\nRc:");
    rc::run();
    println!("\nRefCell:");
    ref_cell::run();
}
