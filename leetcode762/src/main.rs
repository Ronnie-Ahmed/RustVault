pub fn count_prime_set_bits(left: i32, right: i32)->i32 {
    let mut times=0;
        for i in left..=right{
            let  binary_str:Vec<char>=format!("{:b}",i).chars().collect();
            let  count=binary_str.iter().filter(|&&x| x=='1').count();
            if is_prime(count as i32)==true{
                times+=1;
            }
        }
        times
 }

fn main() {
    println!("{}",count_prime_set_bits(10,15));
    
}

pub fn is_prime(number:i32)->bool{
    let mut is_prime=true;
    if number==1{
        return false
    }
    
    for i in 2..number{
        if number%i==0{
            is_prime=false;
            break;
        }
    }
    return is_prime
}
