pub fn power_of_four(n: i32) -> bool {
    if n == 1 {
        return true;
    } else if n <= 0 {
        return false;
    }
    let mut n = n as f32;

    while n >= 4.0 {
        let temp = n / 4.0;
        if temp == 1.0 {
            return true;
        }
        n = temp;
    }

    false
}

pub fn reverse_string(s: &mut Vec<char>) {}
fn main() {
    println!("{}", power_of_four(64));

    let mut temp: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    temp.reverse();
    println!("{:?}", temp);
}
