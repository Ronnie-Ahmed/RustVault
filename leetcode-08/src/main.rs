use std::result;

pub fn my_atoi(s: String) -> i32 {
    let mut chars = s.chars().peekable();
    println!("{:?}", chars);

    // 1. skip leading whitespace
    while chars.peek() == Some(&' ') {
        chars.next();
    }

    // 2. read the optional sign
    let mut sign: i64 = 1;
    match chars.peek() {
        Some('-') => {
            sign = -1;
            chars.next();
        }
        Some('+') => {
            chars.next();
        }
        _ => {}
    }

    // 3. read digits manually, stop at the first non-digit
    let mut result: i64 = 0;
    while let Some(&c) = chars.peek() {
        if !c.is_ascii_digit() {
            break;
        }
        result = result * 10 + (c as u8 - b'0') as i64;

        // 4. clamp as soon as we pass the limits
        if sign * result >= i32::MAX as i64 {
            return i32::MAX;
        }
        if sign * result <= i32::MIN as i64 {
            return i32::MIN;
        }
        chars.next();
    }

    (sign * result) as i32
}

pub fn my_atoi2(s: String)->i32{
            let mut temp_chars=s.chars().peekable();

            while temp_chars.peek()==Some(&' '){
                temp_chars.next();
            }

            let mut sign =1;

            match temp_chars.peek() {
                Some('-')=>{
                    sign=-1;
                    temp_chars.next();
                },
                Some('+')=>{
                
                    temp_chars.next();
                },
                _ => {}
            }

            let mut result=0;

            while let Some(&c) =temp_chars.peek()  {
                if !c.is_ascii_digit(){
                    break;
                }
                result=result * 10 +(c as u8 - b'0') as i64;

                if sign*result >= i32::MAX as i64{
                    return i32::MAX;
                }
                if sign * result <=i32::MIN as i64{
                    return i32::MIN
                }

            }
            (sign * result) as i32
}

fn main() {
    println!("{}", my_atoi("Hello, world!".to_string()));
}
