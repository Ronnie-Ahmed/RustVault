
pub fn move_zeros(nums:&mut Vec<i32>)->Vec<i32>{
    let length=nums.len();

    for  i in 0..length{
        if  nums[i]==0{
            for j in i..length-1{
                let temp=nums[j];
                nums[j]=nums[j+1];
                nums[j+1]=temp;
            }
            
        }
     

        
    }
    nums.to_vec()
}

pub fn move_zeros3(nums:&mut Vec<i32>)->Vec<i32>{
    let mut write_ptr=0;
    for i in 0..nums.len(){
        if nums[i]!=0{
            nums.swap(write_ptr, i);
            write_ptr+=1;
        }
    }
    nums.to_vec()
}


fn main() {
    let mut num=vec![0,0,1];
    println!("{:?}",move_zeros3(&mut num));
}// 1 2 0 1
