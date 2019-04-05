mod boxes;
mod deref;
mod drop;
mod memory_leak;
mod rc;
mod ref_cell;
mod weak_ref;

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
    println!("\nMemory Leak:");
    memory_leak::run();
    println!("\nWeak Ref:");
    weak_ref::run();
}
