pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {


   let len=nums.len();
   if len == 0 {
        return vec![];
    }

    let mut result:Vec<String>=Vec::new();
    let mut count=0;

    for i in 0..len{
        if i <count{
            continue;
        }

        for j in i..len{
            let is_last= j+1==len;
            let is_not_consecutive=!is_last && nums[j]+1!=nums[j+1];

            if is_last || is_not_consecutive{
                if i==j{
                    result.push(format!("{}",nums[j]));
                }else{
                    result.push(format!("{}->{}",nums[i],nums[j]));
                }
                count=j+1;
                break;
            }
        }
        
    }
    result
   
}

fn main() {
    let value=summary_ranges(vec![0,1,2,4,5,7]);
    println!("{:?}",value);
}
