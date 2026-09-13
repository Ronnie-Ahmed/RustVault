pub fn missing_number(nums: Vec<i32>) -> i32 {
   let mut temp=nums;
   temp.sort_unstable();
   let mut count=0;
   
    for i in temp{
        if i!=count{
            return count
        }
        count+=1;
    }
    count
}


fn main(){
    let value=missing_number(vec![0,1]);
    println!("{}",value);
}