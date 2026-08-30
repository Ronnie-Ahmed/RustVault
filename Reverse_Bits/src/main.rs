use std::{format, println};


pub fn reverse_bits(n:i32)->i32{
    let binary_str = format!("{:032b}", n as u32);
    let reverse_str:String=binary_str.chars().rev().collect();
    u32::from_str_radix(&reverse_str, 2).unwrap() as i32
    // let mut binary_num=String::new();
    // let binary_str_convert=n.to_string();
    // for num in binary_str_convert.chars(){
    //     let binary_str=match num{
    //         '1' => "0001",
    //         '2' => "0010",
    //         '3' => "0011",
    //         '4' => "0100",
    //         '5' => "0101",
    //         '6' => "0110",
    //         '7' => "0111",
    //         '8' => "1000",
    //         '9' => "1001",
    //         '0' => "0000",
    //         _ => "Error"
    //     };
    //     binary_num.push_str(binary_str);
    // }
    // // let int_binary:String=binary_num.chars().rev().collect();
    // binary_num

  
    // u32::from_str_radix(&int_binary, 2).unwrap() as i32

    
}


fn main() {
    let binary=reverse_bits(43261596);
    println!("{}",binary)
}
