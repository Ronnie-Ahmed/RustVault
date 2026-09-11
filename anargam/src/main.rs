use std::{collections::HashMap, string};

pub fn anargam(s:String,t:String)->bool{
    
    if s.len()!=t.len(){
        return false
    }

    let mut count_s:HashMap<char,usize>=HashMap::new();
    let mut count_t:HashMap<char,usize>=HashMap::new();

    for ch in s.chars(){
        *count_s.entry(ch).or_insert(0)+=1;
    }
    for ch in t.chars(){
        *count_t.entry(ch).or_insert(0)+=1;
    }

    println!("{:?} {:?}",count_s,count_t);
    
    count_s==count_t
}

fn main() {
    let value=anargam(String::from("rat"), String::from("car"));
    println!("{}",value);
}
