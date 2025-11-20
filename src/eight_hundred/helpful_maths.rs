use std::io::stdin;
pub fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut nums: Vec<i32> = input
        .trim()
        .split("+")
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    nums.sort();
    let res = nums
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("+");

    println!("{res}");
}
