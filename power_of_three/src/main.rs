pub fn is_power_of_three(mut n: i32) -> bool {
    if n == 1 {
        return true;
    } else if (n <= 0) {
        return false;
    }

    let mut temp = n as f64;

    while temp >= 3.0 {
        temp = temp / 3.0;
        if temp == 1.0 {
            return true;
        }
    }
    false
}

fn main() {
    println!("Hello, world!");
    let value = is_power_of_three(3);
    println!("{}", value)
}
