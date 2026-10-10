pub fn max_area(height:Vec<i32>)->i32{
    if height.len()==2{
        return height[0].min(height[1]);
    }

    if height.len() ==3{
        let middle=height[1];
        if middle < height[2] && middle > height[0]{
            return middle
        }else if middle < height[0] && middle <height[2] {
            let mini=height[0].min(height[2]);
            return mini * 2
        }else  {
            let maxim=height[0].max(height[2]);
            return middle * maxim

        }
    }
    let middle=height.len()/2;

    let mut first_half=0;
    let mut first_index=0;
    for i in 0..middle{
        if first_half<height[i]{
            first_half=height[i];
            first_index=i;
        }
    }

    let mut first_check=0;
    
    for i in 0..first_index{
        first_check+=height[i];
    }

    if first_check>first_half{
        first_half=0;
        for i in 0..first_index{
            if first_half< height[i]{
                first_half=height[i];
                first_index=i;
            }
        }
    }
   println!("{} {}",first_half,first_index);

    let mut second_half=0;
    let mut second_index=0;
    for i in middle..height.len(){
        if second_half<height[i]{
            second_half=height[i];
            second_index=i;
        }
    }

    let mut second_check=0;
    
    for i in second_index+1..height.len(){
        second_check+=height[i];
    }

    if second_check>second_half{
        second_half=0;
        for i in second_index+1..height.len(){
            if second_half< height[i]{
                second_half=height[i];
                second_index=i;
            }
        }
    }
    let limit=first_half.min(second_half);
    println!("{:?}",limit);

    // let mut result=0;
    let diff=(second_index-first_index);
    println!("{}",diff);
    // for i in first_index..=second_index{
    //     if height[i]>limit{
    //         result+=limit;
    //     }else {
    //         result+=height[i];
    //     }
        

    // }
    






    println!("{} {}",second_half,second_index);

    (second_index-first_index) as i32 * limit

}

//  [7,9,8,6,2,5,4,8,7]
 //  1 5 9
  // 5 1 9
  // 1 9 5


fn main() {
    println!("Hello, world!");
    let  height=vec![1,2,1];
    println!("{:?}",max_area(height));
}

// 1 2 1
// 