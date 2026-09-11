 pub fn is_power_of_two(n: i32) -> bool {
    if n <=64{
        for i in 0.. n{
            let result=1<<i;
            if result ==n{
                return true
            }
        }
    }
    for i in 0..n/8{
        let result=1 << i;
        if result==n{
            return true;
        }
    }
    false
    }

fn main() {
   let value=power_of_two(3);
    println!("{}",value);
}

