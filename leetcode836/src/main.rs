 pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let x_overlap = rec1[0] < rec2[2] && rec2[0] < rec1[2];
        let y_overlap = rec1[1] < rec2[3] && rec2[1] < rec1[3];

        x_overlap && y_overlap
    }


fn main() {
    println!("{:?}",is_rectangle_overlap(vec![0, 0, 2, 2],vec![1, 1, 3, 3]));
}
