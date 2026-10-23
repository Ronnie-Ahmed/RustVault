pub fn dominant_index(nums: Vec<i32>) -> i32 {
    let mut temp: Vec<i32>=nums.clone();
    temp.sort_unstable();
    let d=temp.len() -1;
    let mut   largest=0;
    if temp[d] >= 2 * temp[d-1]{
        largest=temp[d];
    }else{
        return -1
    }
   let index= nums.iter().position(|i| *i==largest).unwrap();
    index as i32
}
pub fn dominant_index2(nums: Vec<i32>) -> i32 {
    let mut largest=nums[0];
    let mut max_index=0;
    let mut second=0;
    for (i ,&x) in nums.iter().enumerate().skip(1){
        if x >largest{
            second=largest;
            largest=x;
            max_index=i
        }else if x > second{
            second=x
        }
    }
    if largest >= second*2 {return max_index as i32}else{return -1}
}


fn main() {
    println!("{}",dominant_index(vec![1,2,3,4]));
}
