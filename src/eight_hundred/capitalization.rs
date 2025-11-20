use std::io::stdin;
pub fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().to_string();
    if let Some(first) = input.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    println!("{input}")
}
