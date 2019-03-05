mod math;
mod nary;

// bring sum, diff into scope
use math::functions::binary::{sum, diff};

// however.. the idiom for functions is to bring module into scope:
use math::functions::binary;

// even more succinct for last two imports:
// use math::functions::binary::{sum, diff, self};

// bring everything from binary module into scope
// use math::functions::binary::*;



// it is idiomatic to bring a struct, rather than its parent, into scope:
// use std::collections::HashSet as HS;

fn main() {
    let a = 5;
    let b = -3;

    // Absolute path
    println!("sum(a, b): {}", crate::math::functions::binary::sum(a, b));

    // Relative path
    println!("diff(a, b): {}", crate::math::functions::binary::diff(a, b));

    // From Scope:
    println!("norm(sum(a, b), diff(a, b)): {}", binary::l1_norm(sum(a, b),
                                                                diff(a, b)));

    println!("norm({}, {}): {}", a, b, binary::l1_norm(a, b));
    println!("sum(a, b) from nary: {}", nary::binary::sum(5, 3));
}
