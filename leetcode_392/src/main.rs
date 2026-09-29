 pub fn is_subsequence(s: String, t: String) -> bool {

    let  s_temp:Vec<char>=s.chars().collect();
    let   t_temp:Vec<char>=t.chars().collect();
    let mut temp=String::new();
    let mut prev_index=0;
    for i in 0..s_temp.len(){
        for j in prev_index..t_temp.len(){
            if s_temp[i]==t_temp[j]{
                temp.push(s_temp[i].clone());
                prev_index=j+1;
                break;
            }
        }
    }
    if temp==s{
        return true
    }else {
        false
    }
}

fn main() {
    let value=is_subsequence("aza".to_string(),"abzba".to_string());
    println!("{}",value);
}
