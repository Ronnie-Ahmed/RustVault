
 use std::collections::HashMap;
 pub fn find_the_difference(s: String, t: String) ->char{
    let mut temp_s:Vec<char>=s.chars().collect();
    let mut temp_t:Vec<char>=t.chars().collect();
    // temp_s.sort_unstable();
    // temp_t.sort_unstable();
    for i in 0..temp_s.len(){
        for j in 0..temp_t.len(){
            if temp_s[i]==temp_t[j]{
                temp_t.remove(j);
                break;
            }
        }
    }
    temp_t[0]



    // if temp_s.len() < temp_t.len(){
    //     let tlen=temp_t.len();
    // }

    // count_s.remove(k)
    // println!("{:?}",is_subset);

    // temp_t[temp_t.len()]



}

fn main() {
    find_the_difference("ae".to_string(),"aea".to_string());
}
