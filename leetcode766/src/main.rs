pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for i in 1..matrix.len(){
            for j in 1..matrix[0].len(){
                if matrix[i][j]!=matrix[i-1][j-1]{
                    return false
                }
            }
        }
        true
}
fn main() {
    println!("{}",is_toeplitz_matrix(vec![[1,2,3,4].to_vec(),[5,1,2,3].to_vec(),[9,5,1,2].to_vec()]));
}
