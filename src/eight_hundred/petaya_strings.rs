use std::{cmp, io::stdin};
pub fn main() {
    let mut first = String::new();
    let mut second = String::new();

    stdin().read_line(&mut first).expect("Failed to read line");
    stdin().read_line(&mut second).expect("Failed to read line");

    let mut res = 0;
    for (c1, c2) in first.trim().chars().zip(second.trim().chars()) {
        res = match c1.to_ascii_lowercase().cmp(&c2.to_ascii_lowercase()) {
            cmp::Ordering::Greater => 1,
            cmp::Ordering::Less => -1,
            cmp::Ordering::Equal => continue,
        };
        break;
    }
    println!("{res}")
}
