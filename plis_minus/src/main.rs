
fn plusMinus(arr:&[i32]){
    let mut countpositive=0;
    let mut countnegative=0;
    let mut countzeros=0;
    for i in arr{
        if *i < 0{
            countnegative+=1;
        }else if *i > 0 {
            countpositive+=1;
        }else {
            countzeros+=1
        }
    }

   let countpositive =countpositive as f64/arr.len() as f64;
    let countnegative =countnegative as f64/arr.len() as f64;
     let countzeros =countzeros as f64/arr.len() as f64;
     println!("{:.6} \n {:.6} \n {:.6}",countpositive,countnegative,countzeros)

}

fn main() {
   plusMinus(&[-4, 3, -9, 0, 4, 1]);
}
