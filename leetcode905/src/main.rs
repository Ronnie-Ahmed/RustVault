
pub fn sort_array_by_parity(nums: Vec<i32>)->Vec<i32> {
    let mut temp=nums.clone();
        // for (i,j) in nums.iter().enumerate(){
        //     if j%2!=0{
        //         temp.push(*j);
                
        //         temp.remove(i);
        //     }
        // }

        let mut count=0;
        let mut i=0;
        while count < nums.len()-1{


            if temp[i]%2!=0 && temp[i]!=0{
                temp.push(temp[i]);
                temp.remove(i);
            }else if temp[i]==0 || temp[i] % 2==0{
                i+=1;
            }
            count+=1;
        }


        temp
    }
fn main() {
    println!("{:?}",sort_array_by_parity(vec![0,2,1,4]));
}
