
fn staircase(n:i32){
    let mut count=n-1;
    for i in 0..n{
        
        for j in 0..n{

            if j < count{
                print!(" ")
            }else{
                print!("#")
            }

            
        }
        println!();
        count-=1;
        
    }
}



fn main() {
    staircase(6);
}

//      #
//     ##
//    ###
//   ####
//  #####
// ######