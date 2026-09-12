fn add_digits(num: i32) -> i32 {
    if num < 10 {
        return num;
    }
    let mut temp = num;
    let mut temp_vec = Vec::new();
    while temp >= 10 {
        while temp > 0 {
            temp_vec.push(temp % 10);
            temp /= 10;
        }
    
        temp = temp_vec.iter().sum();
        temp_vec.clear();
    }
    temp
}
// 38 3,   8 8

fn main() {
//     println!("Hello, world!");

//     let test: Vec<i32> = vec![1, 2, 3, 4];
//     let add: i32 = test.iter().sum();
//     println!("{}", add);

println!("{}",add_digits(38));
}
