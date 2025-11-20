use std::collections::HashSet;
use std::io::stdin;
pub fn main() {
    let mut input = String::new();
    let mut set: HashSet<char> = HashSet::new();
    stdin().read_line(&mut input).unwrap();
    for c in input.trim().chars() {
        set.insert(c);
    }
    if set.len() % 2 == 0 {
        println!("CHAT WITH HER!")
    } else {
        println!("IGNORE HIM!")
    }
}
