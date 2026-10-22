
pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let mut temp_ch: String = license_plate
        .chars()
        .filter(|i| i.is_alphabetic())
        .map(|i| i.to_ascii_lowercase())
        .collect();
    println!("{}", temp_ch);
    let mut result=Vec::new();

    for (i, w) in words.iter().enumerate() {
        let mut temp = temp_ch.clone();
        for ch in w.chars() {
            if let Some(index) = temp.find(ch) {
                temp.remove(index);
            }
        }
        if temp.is_empty(){
            result.push(i);
        }
       
    }
    let mut lent=i32::MAX;
    for i in result.clone(){
        if words[i].len()< lent as usize{
            lent=words[i].len() as i32;
        }
    }
    let mut result2=String::new();
    for i in result{
        if words[i].len()==lent as usize{
            result2=words[i].clone();
            break;
        }
    }
    result2



    
}

fn main() {
    println!("{}",shortest_completing_word(
        "1s3 456".to_string(),
        vec![
            "looks".to_string(),
            "pest".to_string(),
            "stew".to_string(),
            "show".to_string(),
        ],
    ));
}
