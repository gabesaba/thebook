fn main() {
    let a = "a bcdefg lol hi".to_string();
    let b = &a[0..2];
    println!("{}", b);
    println!("first word {}", nth_word(&a));
}

// Buggy nth word function
fn nth_word(sentence: &str) -> &str {
    let bytes = sentence.as_bytes();
    for (j, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            return &sentence[0..j]
        }
    }
    &sentence[..]
}
