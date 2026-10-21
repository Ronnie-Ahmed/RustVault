
fn diagonalDifference(arr: &[Vec<i32>])->i32 {
   let mut coun1=0;
   let mut coun2=arr.len()-1;
   let mut result=0;
    let mut result2=0;

    for n in 0..arr.len(){
        for j in 0..arr[n].len(){
            if j==coun1 {
                result+=arr[n][j];
               
            }
            if j==coun2{
                result2+=arr[n][j];
                
            }
        }
         coun1+=1;
         if coun2==0{
            break;
         }
         coun2-=1;
    }
    
    (result-result2).abs()
    
  
}

fn main() {
    let a=[
        vec![11, 2, 4],
        vec![4, 5, 6],
        vec![10, 8, -12]
    ];
    
     let value=diagonalDifference(&a);
    println!("{:?}",value);
}
