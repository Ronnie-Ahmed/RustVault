pub fn reverse(x: i32) {
    let mut temp = x;
    let mut count;
    let mut reverse = 0;
    let mut temp_s=String::new();
    temp_s.push_str(&x.to_string());
    println!("{}",temp_s);
    if temp < 0 {
        temp = temp.abs();
    }
    while temp > 0 {
        count = temp % 10;
        //  if let Some(ch) = char::from_digit(count as u32, 10) {
        //     temp_s.push(ch);
        // }
        reverse = reverse * 10 + count;
        temp = temp / 10;
    }

    // if temp_s[0]=='0'{
    //     temp_s.remove(0);
    // }



    if x < 0 {
        reverse = -reverse;
    }

    
    // if reverse > i32::MAX{
    //     return 0
    // }

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


fn main() {
    println!("Hello, world!");
    reverse(120);
}
