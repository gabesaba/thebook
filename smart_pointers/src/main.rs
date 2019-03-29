fn main() {
    boxes();
}

struct NaturalNumber {
    natural_number: Option<Box<NaturalNumber>>,
}

impl NaturalNumber {
    fn new(num: u32) -> NaturalNumber {
        if num == 0 {
            NaturalNumber {
                natural_number: None,
            }
        } else {
            NaturalNumber {
                natural_number: Some(Box::new(NaturalNumber::new(num - 1))),
            }
        }
    }

    fn val(&self) -> u32 {
        let mut as_u32 = 0;
        let mut natural_number = &self.natural_number;
        loop {
            match natural_number {
                Some(n) => {
                    as_u32 += 1;
                    natural_number = &n.natural_number;
                }
                None => return as_u32,
            }
        }
    }
}

fn boxes() {
    let b = Box::new(42);
    let a = NaturalNumber::new(*b);
    println!("Created new natural number of size {}", *b);
    println!("Val of a: {}", a.val());
}

#[cfg(test)]
mod tests {
    use super::NaturalNumber;
    #[test]
    fn test_natural_nums() {
        for i in 0..1000 {
            let a = NaturalNumber::new(i);
            assert_eq![i, a.val()];
        }
    }
}
