pub mod functions {
    pub mod binary {
        pub fn sum(a: i32, b: i32) -> i32 { a + b }

        pub fn l1_norm(a: i32, b: i32) -> i32 {
            let abs = super::unary::abs;
            abs(a) + abs(b)
        }

        pub fn diff(a: i32, b: i32) -> i32 { a - b }
    }

    mod unary {
        pub fn abs(a: i32) -> i32 {
            a.abs()
        }
    }
}
