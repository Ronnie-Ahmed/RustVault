use std::collections::HashMap;
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut prev_index = 0;
    let mut check: HashMap<i32, usize> = HashMap::new();
    for (i, value) in nums.iter().enumerate() {
        if check.contains_key(value) {
            let prev_value = check.get(value).unwrap() + prev_index;
            let diff = i.abs_diff(prev_value);
            prev_index = diff;
            println!(
                "Value {} seen again! Index difference: {} , prev_index: {}",
                value, diff, prev_value
            );
            if diff <= k as usize {
                return true;
            }
        } else {
            check.insert(*value, i);
        }
    }
    false
}

pub fn contains_nearby_duplicate2(nums: Vec<i32>, k: i32) -> bool {
    for i in 0..nums.len() {
        for j in (0..i).rev() {
            if nums[j] == nums[i] {
                let diff = i.abs_diff(j);
                if diff <= k as usize {
                    return true;
                }
            }
        }
    }
    return false;
}
pub fn contains_nearby_duplicate3(nums: Vec<i32>, k: i32) -> bool {
    let mut check: HashMap<i32, usize> = HashMap::new();
    for (current_index, &value) in nums.iter().enumerate() {
        if let Some(&prev_index) = check.get(&value) {
            let diff = current_index - prev_index;
            if diff <= k as usize {
                return true;
            }
        }

        check.insert(value, current_index);
    }
    false
}

fn main() {
    let value = contains_nearby_duplicate3(vec![1, 0, 1, 1], 1);
    println!("{}", value);
}
