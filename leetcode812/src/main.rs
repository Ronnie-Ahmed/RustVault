pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let n = points.len();
        let mut max_area: f64 = 0.0;

        // Loop through every possible combination of 3 distinct points
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    let x1 = points[i][0] as f64;
                    let y1 = points[i][1] as f64;
                    let x2 = points[j][0] as f64;
                    let y2 = points[j][1] as f64;
                    let x3 = points[k][0] as f64;
                    let y3 = points[k][1] as f64;

                    // Triangle area formula using coordinates (Shoelace formula)
                    let area = 0.5 * ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2))).abs();

                    if area > max_area {
                        max_area = area;
                    }
                }
            }
        }

        max_area
    }

fn main() {
    println!("{:?}",largest_triangle_area(vec![vec![0,0],vec![0,1],vec![1,0],vec![0,2],vec![2,0]]));
}
