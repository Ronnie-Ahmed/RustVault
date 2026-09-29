
/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

 struct Solution;

 fn guess(num:i32)->i32{
    12
 }

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        
        let mut low=1;
        let mut high=n;
        let mut middle = low + (high - low) / 2;
       while let num=guess(middle){
        if num==0{
            break;
        }else if num==-1{
            high=middle;
            middle = low + (high - low) / 2;
        }else{
            low=middle+1;
           middle = low + (high - low) / 2;
        }
       }
       middle

    }
}

// -1 if num is higher
// 1 if num is smaller
// 0 if equal

fn main() {
    println!("Hello, world!");
}
