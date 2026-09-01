use std::{collections::HashSet, println};

// pub fn happy_number(n:i32)->bool{
//     let mut value=n;
//     let mut first_number=0;
//     let mut second_number=0;
//     let mut ask:bool=true;
//     while value !=0 {
//         first_number=value;
//         second_number= value % 10;
//         value= (first_number * first_number) + (second_number * second_number);
//         println!("{}",value);
//         if value==1{
//             ask= true;
//             break;
//         }
//         if (value==n){
//             ask= false;
//             break;
//         }
//     }
//     ask
// }

pub fn happy_number(n: i32) -> bool {
    if n == 1 || n == 7 {
        return true;
    }
    let mut i = n;
    let mut value = false;
    while i > 9 {
        i = i
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .map(|x| x * x)
            .sum();
        if i == 1 {
            value = true;
            break;
        }
        if i == n {
            value = false;
            break;
        }
    }

    value
}
// 19 = 9

pub fn is_happy(n: i32) -> bool {
   let mut seen =HashSet::new();
   let mut num=n;
   while num !=1 && !seen.contains(&num){
        seen.insert(num);
        num=squared_number(num);
   }
   n==1
}

pub fn squared_number(mut num: i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        let digit = num % 10;
        sum += (digit * digit);
        num = num / 10
    }
    sum
}

fn main() {
    let see = happy_number(19);
    println!("{}", see);

    // let  j=19;
    // let mut i=j;
    // let mut value=false;
    // while i > 9{
    //     i=i.to_string().chars().filter_map(|c| c.to_digit(10)).map(|x| x*x).sum();
    //     if i==1{
    //         value= true;
    //         break;
    //     }
    //     if i==j{
    //         value=false;
    //         break;
    //     }
    // }
    // println!("{}",value)
}
