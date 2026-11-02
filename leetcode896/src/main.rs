pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut count=i32::MIN;
    if nums[0] <= nums[nums.len()-1]{
        // Increasing
        for i in nums{
            if i >= count{
                count=i
            }else{
                return false
            }
        }
    }else{
        for i in nums.iter().rev(){
            if *i >=count{
                count=*i
            }else{
                return false
            }
        }
    }
    true
}


fn main() {
    println!("{:?}",is_monotonic(vec![1,3,2]))
}
