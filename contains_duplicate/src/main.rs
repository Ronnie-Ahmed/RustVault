use std::collections::HashSet;
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut check = HashSet::new();
    for num in &nums {
        if check.contains(num) {
            return true;
        } else {
            check.insert(num);
        }
    }
    return false;
}
pub fn contains_duplicate2(nums: Vec<i32>) -> bool {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                return true;
            }
        }
    }
    return false;
}

pub fn contains_duplicate3(nums: Vec<i32>) -> bool {
    let mut seen=HashSet::with_capacity(nums.len());
    !nums.into_iter().all(|i| seen.insert(i))
}
pub fn contains_duplicate4(nums: Vec<i32>) -> bool {
    let mut value=nums.clone();
    value.sort_unstable();
    value.windows(2).any(|w| w[0] == w[1])
}


fn main() {
    let value = contains_duplicate3(vec![1, 2, 3, 4]);
    println!("{}", value);
}
