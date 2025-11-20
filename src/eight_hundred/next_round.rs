use std::io;

pub fn main() {
    let mut res: i32 = 0;

    let mut init = String::new();
    io::stdin().read_line(&mut init).unwrap();

    let v: Vec<&str> = init.trim().split_whitespace().collect();

    let placement: usize = v[1].parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let scores: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for n in &scores {
        res += if *n >= scores[placement - 1] && *n > 0 {
            1
        } else {
            0
        }
    }
    println!("{res}")
}
