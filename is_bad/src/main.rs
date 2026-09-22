// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

// impl Solution {
//     pub fn first_bad_version(&self, n: i32) -> i32 {
//         let mut left = 1;
//         let mut right = n;

//         while left < right {
//             // Avoid potential integer overflow with left + (right - left) / 2
//             let mid = left + (right - left) / 2;

//             if self.isBadVersion(mid) {
//                 // mid could be the first bad version, search left half including mid
//                 right = mid;
//             } else {
//                 // mid is good, so the first bad version must be after mid
//                 left = mid + 1;
//             }
//         }

//         left
//     }
// }

fn main() {
    println!("Hello Worlds")
}
