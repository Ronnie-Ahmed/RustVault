pub fn counting_bits(n: i32) -> Vec<i32> {
    let mut temp: Vec<i32> = Vec::new();

    for mut i in 0..n + 1 {
        let mut binary_str = String::new();
        while i > 0 {
            let remainder = i % 2;
            binary_str.push_str(&remainder.to_string());
            i /= 2;
        }

        // println!("{}",binary_str);
        let mut temp2: Vec<char> = binary_str.chars().filter(|c| *c == '1').collect();
        // println!("{:?}",temp2);
        temp.push(temp2.len() as i32);
        temp2.clear();
    }

    temp
}

fn main() {
    println!("{:?}", counting_bits(5));
}
