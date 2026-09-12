

   pub fn is_ugly(n: i32) -> bool {
    // Ugly numbers must be strictly positive (> 0)
    if n <= 0 {
        return false;
    }
    let mut temp=n;

    // Repeatedly divide by 2, 3, and 5
    for factor in [2, 3, 5] {
        while temp % factor == 0 {
            temp /= factor;
        }
    }

    // If fully reduced to 1, all prime factors were 2, 3, or 5
    temp == 1
    
    }



pub fn in_prime(num:i32)->bool{
    if num==1 {
        return false;
    }
    let mut prime=true;

    for i in 2.. num{
        if num%i == 0{
            prime=false;
            break;
        }
    }
    prime
}

fn main() {
    let value=is_ugly(7);
    println!("{}",value);
}
