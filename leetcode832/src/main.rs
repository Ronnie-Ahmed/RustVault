pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut temp = image.clone();
    for i in 0..temp.len() {
        temp[i].reverse();
        for j in 0..temp[i].len(){
             if temp[i][j] == 0 {
                temp[i][j] = 1;
            } else if temp[i][j]==1{
                temp[i][j] = 0;
            }
        }
    }
    temp
}

fn main() {
    let image: Vec<Vec<i32>> = vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 1, 1, 1], vec![1, 0, 1, 0]];
    println!("{:?}", flip_and_invert_image(image));
}
