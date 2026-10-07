use std::cmp::Ordering;

pub fn reverse(x: i32) -> i32 {
    if x == 0 || (x > 0 && x < 10) {
        return x;
    }
    let mut temp_s = x.unsigned_abs().to_string();
    let mut reverse_s = String::new();

    for i in temp_s.chars().rev() {
        reverse_s.push(i);
    }
    while reverse_s.starts_with('0') {
        reverse_s.remove(0);
    }
    let compare_numeric_strings =
        |a: &str, b: &str| -> Ordering { a.len().cmp(&b.len()).then_with(|| a.cmp(b)) };
    let limit = if x > 0 {
        i32::MAX.to_string()
    } else {
        i32::MIN.to_string()
    };
    if compare_numeric_strings(&reverse_s, &limit) == Ordering::Greater {
        return 0;
    }

    if x < 0 {
        reverse_s.insert(0, '-');
    }
    reverse_s.parse::<i32>().unwrap_or(0)
}

pub fn reverse3(x: i32) -> i32 {
    let s: String = x.to_string().chars().rev().filter(|&c| c != '-').collect();
    let n: i32 = s.parse().unwrap_or(0); // overflow -> 0
    if x < 0 { -n } else { n }
}


// 123
// 30
// 12
/*
first take a string
push the reverse value into the string
remove first value fo the string using while loop if it is zero , temp_s[0]=='0' , temp_s.remove(0)
now compare the reverse string with string with (i32::MAX) only if the len of both string is same, compare each character , compare first character if reverse is bigger then return 0 , if it is equal compare the next char .. Do it until the very last character
return the value
*/
    pub fn reverse2(x: i32) -> i32 {
         if x==0  || (x>0 && x < 10){
        return x
    }
    let mut temp_num = x;
    let mut temp_s = String::new();
    let mut reverse_s = String::new();
    if temp_num < 0 {
        temp_num = temp_num.abs();
    }
    temp_s.push_str(&temp_num.to_string());
    for i in temp_s.chars().rev() {
        reverse_s.push(i);
    }
    while reverse_s.starts_with('0') {
        reverse_s.remove(0);
    }
    let compare_numeric_strings =
        |a: &str, b: &str| -> Ordering { a.len().cmp(&b.len()).then_with(|| a.cmp(b)) };
    let mut u = 0;
    let mut y=0;
    let max: i32 = i32::MAX;
    let min = i32::MIN;
    let mut temp_max = String::new();
    let mut temp_min = String::new();
    temp_max.push_str(&max.to_string());
    temp_min.push_str(&min.to_string());

    if reverse_s.len() == temp_max.len() || reverse_s.len()==temp_min.len(){
        if x > 0 {
        match compare_numeric_strings(&reverse_s, &temp_max) {
            Ordering::Greater => u += 1,
            Ordering::Equal => u += 0,
            Ordering::Less => u -= 1,
        }
    } else if x < 0 {
        reverse_s.insert(0, '-');
        match compare_numeric_strings(&reverse_s, &temp_min) {
            Ordering::Greater => y+= 1,
            Ordering::Equal => y += 0,
            Ordering::Less => y -= 1,
        }
    }
    }

    if u == 1 || y== -1{
        return 0
    } else if x < 0 {
        reverse_s.insert(0, '-');
    }
     
    match reverse_s.parse::<i32>() {
        Ok(num) => num,
        Err(_) => 0,
    }
    }



fn main() {
    println!("Hello, world!");
    let value = reverse(-2147483412);

    println!("{}", value);
}
// -2147483648 2147483647
// -2143847412
