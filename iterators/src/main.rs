fn main() {
    println!("Hello, world!");
    print_sum_of_squares_through_10e6();
    print_even_squares();
}

fn print_even_squares() {
    let a: Vec<_> = (0..25).map(|x| x * x).filter(|x| x % 2 == 0).collect();
    println!("even squares: {:?}", a)
}

fn print_sum_of_squares_through_10e6() {
    let squares = Squares::new(10e6 as u64);

    // Note: don't have to define squares as mutable because sum takes ownership
    // and makes mutable in background (same happens with for loop)
    let sum: u64 = squares.sum();
    println!("Sum of squares through e6: {}", sum);
}

struct Squares {
    curr: u64,
    end: u64,
}

impl Squares {
    fn new(end: u64) -> Squares {
        Squares { curr: 1, end }
    }
}

impl Iterator for Squares {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let ans = self.curr * self.curr;
        if ans <= self.end {
            self.curr += 1;
            return Some(ans);
        } else {
            return None;
        }
    }
}

#[test]
fn test_squares() {
    let mut sq = Squares::new(9);

    assert_eq!(sq.next(), Some(1));
    assert_eq!(sq.next(), Some(4));
    assert_eq!(sq.next(), Some(9));
    assert_eq!(sq.next(), None);
}

#[test]
fn test_sum_sq() {
    let sq = Squares::new(16);

    assert_eq!(30u64, sq.sum());
}
