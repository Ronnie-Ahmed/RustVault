pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut temp: Vec<i32> = Vec::new();
    let mut temp2: Vec<i32> = Vec::new();
    if nums1.len() < nums2.len() {
        temp = nums1.clone();
        temp.sort_unstable();
        while temp.len() != 0 {
            let num = temp[0];
            if nums2.contains(&num) {
                temp2.push(num);
                temp.remove(0);
                
            } else {
                temp.remove(0);
            }
        }
    } else {
        temp = nums2.clone();
        temp.sort_unstable();
        temp.dedup();
        while temp.len() != 0 {
            let num = temp[0];
            if nums1.contains(&num) {
                temp2.push(num);
                temp.remove(0);
            } else {
                temp.remove(0);
            }
        }
    }
    temp2
}

// use std::collections::HashMap;
// pub fn intersection3(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
//     let mut temp5: Vec<i32> = Vec::new();
//     let mut temp:HashMap<i32,i32>=HashMap::new();
//     let mut temp1:HashMap<i32,i32>=HashMap::new();

//     for i in 0..nums1.len(){
//         temp1.insert(i as i32, nums1[i]);
//     }
//       for i in 0..nums2.len(){
//         temp.insert(i as i32, nums1[i]);
//     }
    
//     temp1.



//     temp5
// }

use std::collections::HashSet;
pub fn intersection2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    
    let mut temp2:HashSet<i32>=nums2.into_iter().collect();
    let mut seen: HashSet<i32>=HashSet::new();
    let mut result:Vec<i32>=Vec::new();
    for num in nums1{
        if temp2.contains(&num) && seen.insert(num){
            result.push(num);
        }
    }
    result

}


fn main() {
    println!("{:?}", intersection2(vec![1,2,2,1], vec![2,3]));
}
