use std::io::stdin;
pub fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let sizes: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let res = (sizes[0] * sizes[1]) / 2;
    println!("{res}")
}
