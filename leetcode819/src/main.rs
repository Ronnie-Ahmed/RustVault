use std::{collections::HashMap, result};

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let mut temp:HashMap<String,usize>=HashMap::new();
    // println!("{:?}",paragraph.split_whitespace());
    // let  temp_char=vec![',','.','!'];
    let  temp_str:String=paragraph.chars().map(|c| if c.is_ascii_alphabetic(){c}else{' '}).collect();
    
    
    println!("{:?}",temp_str);
    for word in temp_str.split_whitespace(){
        let  lowercase=word.to_lowercase();
        
        
        println!("{:?}",lowercase);
        *temp.entry(lowercase).or_insert(0)+=1;
    }

    let mut sorted:Vec<(String,usize)>=temp.clone().into_iter().collect();
    sorted.sort_by(|a,b|b.1.cmp(&a.1));
    println!("{:?}",sorted);
    let mut result=String::new();
    for counts in &sorted{
        if !banned.contains(&counts.0){
            result=counts.0.clone();
            break;
        }
    }
        
result
}


fn main() {
    println!("{:?}",most_common_word("Ba, a, a, a, b,b,b,c, c".to_string(),vec!["a".to_string()]));
}
