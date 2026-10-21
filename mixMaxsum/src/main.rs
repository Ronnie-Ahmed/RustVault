
fn miniMaxSum2(arr: &[i32]) {
    let mut temp:Vec<i64>=Vec::new();

    for i in 0..arr.len(){
        let mut sum:i64=0;
        for j in 0..arr.len(){
            if i==j{
                continue;
            }else{
                sum+=arr[j] as i64;
            }
        }
        temp.push(sum);
    }
    let max=*temp.iter().max().unwrap();
    let min=*temp.iter().min().unwrap();

    println!("{} {}",min,max);
}

fn miniMaxSum(arr: &[i32]) {
    let min=*arr.iter().min().unwrap();
    let max=*arr.iter().max().unwrap();
    let total:i64=arr.iter().map(|x| *x as i64).sum();
    println!("{} {}",total-max as i64,total-min as i64)
}



fn main() {
    miniMaxSum(&[5 ,5 ,5, 5, 5]);
}
