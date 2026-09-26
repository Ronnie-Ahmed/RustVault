pub fn reverse_volwels(s: String) -> String {
    let mut temp: Vec<char> = s.chars().collect();
    let mut temp_str: Vec<char> = Vec::new();
    for ch in temp.iter() {
        if *ch == 'A'
            || *ch == 'a'
            || *ch == 'e'
            || *ch == 'E'
            || *ch == 'i'
            || *ch == 'I'
            || *ch == 'o'
            || *ch == 'O'
            || *ch == 'u'
            || *ch == 'U'
        {
            temp_str.push(ch.clone());
        }
    }

    temp_str.reverse();

    let mut count = 0;
    for i in 0..temp.len() {
        if temp[i] == 'A'
            || temp[i] == 'a'
            || temp[i] == 'e'
            || temp[i] == 'E'
            || temp[i] == 'i'
            || temp[i] == 'I'
            || temp[i] == 'o'
            || temp[i] == 'O'
            || temp[i] == 'u'
            || temp[i] == 'U'
        {
            temp[i] = temp_str[count];
            count += 1;
        }
    }
    temp.into_iter().collect()
}

fn main() {
    let result = reverse_volwels("IceCreAm".to_string());
    println!("{:?}", result);
}
