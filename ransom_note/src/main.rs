pub fn can_construct(ransom_note: String, magazine: String) -> bool {

    let  char_ransom:Vec<char>=ransom_note.chars().collect();
    let mut char_magazine:Vec<char>=magazine.chars().collect();
    let len=char_ransom.len();
    let mut temp=String::new();

    for i in 0..len{
        if let Some(index) =char_magazine.iter().position(|&c| c==char_ransom[i]){
            char_magazine.remove(index);
            temp.push(char_ransom[i]);
        }else{
            return false
        }
    }

    if temp==ransom_note{
        return true;
    }else{
        return false;
    }
}

use std::collections::HashMap;

pub fn first_uniq_char(s:String)->i32{
    let mut counts=HashMap::new();
     for c in s.chars(){
        *counts.entry(c).or_insert(0)+=1;
     }

     for (i,ch) in s.chars().enumerate(){
        if counts[&ch]==1{
            return i as i32
        }
     }
     -1
}

fn main() {
    let value=can_construct("aab".to_string(),"baa".to_string());
    let value2=first_uniq_char("leetcode".to_string());
    println!("{}",value2);
}
