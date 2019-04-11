fn main() {
    println!("Hello, world!");
}

struct SummedCollection {
    nums: Vec<i32>,
    sum: i32,
}

impl SummedCollection {
    pub fn new() -> SummedCollection {
        SummedCollection {
            nums: vec![],
            sum: 0,
        }
    }

    pub fn add_num(&mut self, num_to_add: i32) {
        self.nums.push(num_to_add);
        self.sum += num_to_add;
    }

    pub fn remove(&mut self, index: usize) -> i32 {
        let num_to_remove = self.nums.remove(index);
        self.sum -= num_to_remove;
        num_to_remove
    }

    pub fn get_sum(&self) -> i32 {
        self.sum
    }
}

#[cfg(test)]
mod tests {

    use super::SummedCollection;

    #[test]
    fn test_initial_sum_eq_zero() {
        let summed_collection = SummedCollection::new();
        assert_eq!(0, summed_collection.get_sum());
    }

    #[test]
    fn test_add_and_remove() {
        let mut summed_collection = SummedCollection::new();
        assert_eq!(0, summed_collection.get_sum());

        summed_collection.add_num(5);
        assert_eq!(5, summed_collection.get_sum());

        assert_eq!(5, summed_collection.remove(0));

        assert_eq!(0, summed_collection.get_sum());
    }

    #[test]
    fn test_negative_num() {
        let mut summed_collection = SummedCollection::new();
        summed_collection.add_num(-5);
        assert_eq!(-5, summed_collection.get_sum());
        summed_collection.add_num(-7);
        assert_eq!(-12, summed_collection.get_sum());
        summed_collection.add_num(9);
        assert_eq!(-3, summed_collection.get_sum());
        assert_eq!(-7, summed_collection.remove(1));
        assert_eq!(4, summed_collection.get_sum());
    }
}
