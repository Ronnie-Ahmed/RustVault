pub fn reverse_string(s: &mut Vec<char>) {
    s.reverse();
    println!("{:?}", s);
}

fn main() {
    println!("Hello, world!");
    let mut s: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
}
