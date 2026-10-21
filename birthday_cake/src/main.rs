
fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let max=candles.iter().max().unwrap();
    let count:Vec<&i32>=candles.iter().filter(|a| *a==max).collect();
    count.len() as i32
    
}


fn main() {
    println!("{}",birthdayCakeCandles(&[3 ,2, 1, 3]));
}
