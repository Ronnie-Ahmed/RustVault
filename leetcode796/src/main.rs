pub fn rotate_string(s: String, goal: String)->bool {
    let mut temp=s.clone();
    let  len=s.len();
        for _ in 0..len{
            let  c=temp.remove(len-1);
            temp.insert(0, c);
            if temp==goal{
                return true
            }
        }
        false
        
}
fn main() {
    println!("{:?}",rotate_string("abcde".to_string(),"cdeab".to_string()));

    // let mut temp="Ronnie".to_string();
    // let mut c=temp.remove(temp.len()-1);
    // temp.insert(0, c);
    // println!("{}",temp);
}
