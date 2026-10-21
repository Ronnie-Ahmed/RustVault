
fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result:Vec<i32>=Vec::new();
    let mut result_a=0;
    let mut result_b=0;
    for i in 0..3{
        if a[i]>b[i]{
            result_a+=1;
        }else if a[i] < b[i] {
            result_b+=1;
        }else{
            result_a+=0;
            result_b+=0;
        }
    }
    let a=0;
    let b=2;
    
    result.push(result_a);
    result.push(result_b);
    result
}


fn main() {
    let a=vec![17 ,28, 30];
    let b=vec![99 ,16, 8];
    let value=compareTriplets(&a,&b);
    println!("{:?}",value);
}
