mod conditional_impl;

fn main() {
    introduce(English{}, French{});
    println!("Lingua Franca greeting: {}", get_lang().greet());
    let a = conditional_impl::Pair::new(5, 6);
    a.cmp_display();
    let _b = conditional_impl::Pair::new((), ());
    // Not allowed because std::fmt::Display trait not satisfied
    // _b.cmp_display()
}

fn introduce<T, V>(a: T, b: V) where T: Language, V: Language {
    println!("a: {}\nb: {}", a.greet(), b.greet());
}

fn get_lang() -> impl Language {
    LinguaFranca{}
}

pub trait Language {
    fn greet(&self) -> &'static str {
        "Hello"
    }
}

struct English {}
struct French {}
struct LinguaFranca{}

impl Language for English {}
impl Language for LinguaFranca {}
impl Language for French {
    fn greet(&self) -> &'static str {
        "Bonjour"
    }
}

// This would be illegal because of Coherence: either Trait or Struct must be local to our crate
// https://doc.rust-lang.org/error-index.html#E0117
/*
use std::fmt::Display;
impl Display for Vec<i32> {
    fn fmt(&self) -> String {

    }
}
*/
