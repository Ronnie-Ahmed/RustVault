use std::collections::HashMap;
pub fn longest_palindrome(s:String)->i32{
    if s.len()==1{
        return 1
    }
    let  temp_char:Vec<char>=s.chars().collect();
    let mut count=HashMap::new();
    for ch in temp_char{
        *count.entry(ch).or_insert(0)+=1;
    }

    println!("{:?}",count);


    45
}


fn main() {
    println!("Hello, world!");
    longest_palindrome("abccCcdd".to_string());
}
