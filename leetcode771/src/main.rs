   pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let  temp:Vec<char>=jewels.chars().collect();
        let mut count=0;
        for ch in stones.chars(){
            for c in temp.iter(){
                if *c==ch{
                    count+=1;
                }
            }
        }

        count
    }

fn main() {
    println!("{}",num_jewels_in_stones("z".to_string(),"ZZ".to_string()));
}
