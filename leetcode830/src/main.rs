pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let mut temp: Vec<Vec<i32>> = Vec::new();
    let temp_char: Vec<char> = s.chars().collect();
    
    let mut vec_count = 0;
    
    let mut i=0;
    while i < temp_char.len() {
        // println!("{}",i);
        let start = i;
        let mut count = 0;
        let mut end = temp_char.len();
        let check = temp_char[i];
        for j in i..temp_char.len() {
            if check == temp_char[j] {
                 count += 1;
            } else {
                end = j;
                break;
               
                
            }
        }

        if count >= 3 {
            temp.insert(vec_count, vec![start as i32, end as i32 -1]);
            vec_count += 1;
            i=end;
            continue;
        }
        println!("{}",count);
        
       i=end;
        
    }
    temp
}

// abcdddeeeeaabbbcd

fn main() {
    println!(
        "{:?}",
        large_group_positions("babaaaabbb".to_string())
    );
}
