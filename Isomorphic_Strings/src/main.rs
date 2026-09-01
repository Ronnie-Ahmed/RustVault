use std::{collections::HashMap, println};
pub fn is_isomorphic(s:String,t:String)->bool{
    if s.len() != t.len(){
        return false;
    }

    let  s_char:Vec<char>=s.clone().chars().collect();
    let  t_char:Vec<char>=t.clone().chars().collect();

    let mut s_first:HashMap<char,char>=HashMap::new();
    let mut t_second:HashMap<char,char>=HashMap::new();
    for i in 0..s_char.len(){
        let s_val=s_char[i];
        let t_val=t_char[i];
        
        if let Some(&mapped_t) = s_first.get(&s_val) {
            if mapped_t!=t_val{
                return false
            }
            
        }else{
            s_first.insert(s_val, t_val);
        }

        let rev_s_val=s_char[s_char.len()-i-1];
        let rev_t_val=t_char[s_char.len()-i-1];

        if let Some(&mapped_s) = t_second.get(&rev_t_val) {
            if mapped_s!=rev_s_val{
                return false
            }
            
        }else {
            t_second.insert(rev_t_val, rev_s_val);
        }


        
    }

    true
}

// s -> t    e g g t
// t -> s    a d d c

fn main() {
    let value=is_isomorphic(String::from("egg"),String::from("add"));
    println!("{}",value);
}
