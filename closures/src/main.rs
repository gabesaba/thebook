use std::collections::HashMap;
use std::thread;
use std::time;

fn main() {
    let expensive_op = || {
        thread::sleep(time::Duration::from_secs(1));
        5
    };

    expensive_op();

    let successor = |x| x + 1;

    let x = 5;
    println!("The successor of {} is {}", x, successor(x));

    let mut sq_x = Memoizer::new(|x| x * x);
    println!("{}", sq_x.get_val(5));
    println!("{}", sq_x.get_val(5));
    println!("{}", sq_x.get_val(6));

    // FnOnce:
    let x = vec![1, 2, 3];

    let equal_to_x = move |y| y == x;
    let val_to_move = vec![1, 2, 3];
    println!("Result of fn_once: {}", equal_to_x(val_to_move));
    // println!("Input to fn_once: {:?}", val_to_move);
}

struct Memoizer<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    result: HashMap<u32, u32>,
}

impl<T> Memoizer<T>
where
    T: Fn(u32) -> u32,
{
    fn new(f: T) -> Memoizer<T> {
        Memoizer {
            calculation: f,
            result: HashMap::new(),
        }
    }

    fn get_val(&mut self, arg: u32) -> u32 {
        println!("Getting val...");
        match self.result.get(&arg) {
            Some(x) => *x,
            None => {
                thread::sleep(time::Duration::from_secs(2));
                let val = (self.calculation)(arg);
                self.result.insert(arg, val);
                val
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Memoizer;
    use std::time;

    #[test]
    fn test_expected_out() {
        let mut memoizer = Memoizer::new(|x| 5);
        assert_eq!(5, memoizer.get_val(5));
    }

    #[test]
    fn test_different_inputs() {
        let mut memoizer = Memoizer::new(|x| x * x);
        assert_eq!(25, memoizer.get_val(5));
        assert_eq!(36, memoizer.get_val(6));
    }

    #[test]
    fn test_faster_second_time() {
        let mut memoizer = Memoizer::new(|x| x * x);
        let t1 = time::Instant::now();
        memoizer.get_val(5);
        let e1 = t1.elapsed();

        let t2 = time::Instant::now();
        memoizer.get_val(5);
        let e2 = t2.elapsed();

        assert!(e2 < e1);
    }
}
