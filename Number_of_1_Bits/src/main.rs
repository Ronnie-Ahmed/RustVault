pub fn hamming_weights(n:i32)->i32{
    let binary_str=format!("{:b}",n);
    let mut count=0;
    for char in binary_str.chars(){
        if char=='1'{
            count+=1;
        }
    }
    count
}
fn main() {
let count=hamming_weights(2147483645);
    println!("{}",count)
}
