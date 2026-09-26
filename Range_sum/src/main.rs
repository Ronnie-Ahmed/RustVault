use std::path::absolute;

struct NumArray {
    numbers: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray { numbers: nums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut total = 0;
        let mut temp = left;
        while temp <= right {
            total = self.numbers[temp as usize] + total;
            temp += 1;
        }
        total
    }
}

fn main() {
    println!("Hello, world!");
}
