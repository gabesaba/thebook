pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn sqrt(a: f64) -> f64 {
    assert!(a >= 0.0, "What was that all about?!");
    a.sqrt()
}

#[cfg(test)]
mod tests {
    use super::{multiply, sqrt};

    #[test]
    fn eq() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn not_eq() {
        assert_ne!(2 + 2, 5);
    }

    #[test]
    fn test_multiply() {
        for i in 0..100 {
            for j in 0..100 {
                assert_eq!(multiply(i, j), i * j)
            }
        }

    }

    #[test]
    fn it_fails() {
        panic!("ahhh what went wrong?!")
    }

    #[test]
    fn assert_with_custom_err() {
        assert!(false, "It failed because I asserted false :S")
    }

    #[test]
    #[should_panic(expected = "What was that all about?!")]
    fn test_sqrt_with_neg_num() {
        println!("{}", sqrt(-5.0));
    }

    #[test]
    fn test_sqrt_with_result() -> Result<(), String> {
        if sqrt(25.0) == 5.0 {
            Ok(())
        } else {
            Err("That's odd".to_owned())
        }
    }

    #[test]
    fn test_sqrt_with_bad_result() -> Result<(), String> {
        if sqrt(25.0) == 5.1 {
            Ok(())
        } else {
            Err("That's not so odd".to_owned())
        }
    }
}
