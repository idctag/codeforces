use std::io;
pub fn main() {
    let mut times = String::new();
    io::stdin().read_line(&mut times).unwrap();
    let n_times: i32 = times.trim().parse().unwrap();
    for _i in 0..n_times {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let w: String = input.trim().parse().unwrap();
        let chars: Vec<char> = w.chars().collect();
        if w.len() <= 10 {
            println!("{w}")
        } else {
            println!("{}{}{}", chars[0], chars.len() - 2, chars[chars.len() - 1])
        }
    }
}
