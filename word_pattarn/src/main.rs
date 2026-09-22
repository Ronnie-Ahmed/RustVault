use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut map: HashMap<char, String> = HashMap::new();
    let words: Vec<String> = s.split_whitespace().map(|a| a.to_string()).collect();
    let chars: Vec<char> = pattern.chars().collect();

    if words.len() != chars.len() {
        return false;
    }
    let len = chars.len();

    for i in 0..len {
        if map.contains_key(&chars[i]) {
            let temp = map.get(&chars[i]);
            if temp.unwrap() != &words[i] {
                return false;
            }
        } else {
            let target_value = &words[i];

            let value_already_used = map.values().any(|val| val == target_value);

            if value_already_used {
                return false;
            }

            map.insert(chars[i], words[i].clone());
        }
    }
    true
}

fn main() {
    let value = word_pattern("abba".to_string(), "dog cat cat fish".to_string());
    println!("{}", value);
}
