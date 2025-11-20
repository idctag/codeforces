use std::io;

pub fn main() {
    let mut times_string = String::new();
    io::stdin().read_line(&mut times_string).unwrap();
    let times: i32 = times_string.trim().parse().unwrap();

    let mut res: i32 = 0;

    for _ in 0..times {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let a: Vec<char> = input.chars().filter(|c| c != &' ').collect();
        let curr = a.iter().filter(|&&c| c == '1').count();
        res += if curr >= 2 { 1 } else { 0 }
    }
    println!("{res}")
}
