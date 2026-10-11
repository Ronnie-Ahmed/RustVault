use std::{collections::HashSet, vec};
pub fn length_of_longest_substring2(s:String)->i32{
    let  temp_ch:Vec<char>=s.chars().collect();
    let mut temp:HashSet<char>=HashSet::new();
    for ch in temp_ch{
        temp.insert(ch);
    }
    temp.len() as i32
  
}

pub fn length_of_longest_substring3(s:String)->i32{
    if s.len()==1{
        return 1
    }
    let mut temp_ch:Vec<char>=s.chars().collect();
    let temp=temp_ch[temp_ch.len()-1];
    temp_ch.push(temp);
    let mut temp_ch_2: Vec<char>=Vec::new();
    let mut count=0;
    let mut temp_vec:Vec<i32>=Vec::new();
    for i in 0..temp_ch.len()-1{
        if temp_ch[i]==temp_ch[i+1] {
            if !temp_ch_2.contains(&temp_ch[i]){
                count+=1;
            }
            
            temp_vec.push(count);    
        }else{
            if temp_ch_2.contains(&temp_ch[i]){
               temp_vec.push(count);
                
            }else{
                 count+=1;
                 temp_vec.push(count);
            temp_ch_2.push(temp_ch[i]);
            }
           
        }
    }
//    let mut  value=temp_vec[0];
//    for i in temp_vec{
//         if i > value{
//             value=i;
//         }
//    }
//    value
    temp_vec.iter().map(|&i| i).max().unwrap()
    
}

pub fn length_of_longest_substring(s:String)->i32{
    let temp_char:Vec<char>=s.chars().collect();
    let mut set=HashSet::new();
    let mut left=0;
    let mut best=0;
    for right in 0..temp_char.len(){
        while set.contains(&temp_char[right]) {
            set.remove(&temp_char[left]);
            left+=1;
        }
        set.insert(&temp_char[right]);
        best=best.max(right-left+1);
    }
    
    best as i32
}


/// abc

fn main() {
    println!("{:?}",length_of_longest_substring("bbbbb".to_string())) ;

    // let mut test:Vec<i32>=vec![1,2,3,4,2,2,222,3,3,4,222,3,31212,3,4];
    // let  value=test.iter().map(|&i| i).max().unwrap();
    // println!("{}",value);


}
