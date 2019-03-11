fn main() {
    println!("Hello, world!");
    let a = vec![1, 2, 3, 4, 5];
    println!("{}", largest(&a));

    // This wont work because Point expects same type
    // let point = Point{x: 5, y: -5.0};

    let point = Point{x: 0, y: 5};
    let h_point = HeterogeneousPoint{ x: 5, y: -5.0};
}


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &i in list.iter() {
        if i > largest {
            largest = i
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct HeterogeneousPoint<T, V> {
    x: T,
    y: V,
}

// From Book
impl<T, V> HeterogeneousPoint<T, V> {
    fn mixup<U, W>(self, other: HeterogeneousPoint<U, W>) -> HeterogeneousPoint<T, W> {
        HeterogeneousPoint {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}
