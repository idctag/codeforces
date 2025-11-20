use std::io;

pub fn main() {
    let mut x: i32 = 0;
    let mut init = String::new();
    io::stdin().read_line(&mut init).unwrap();

    let times: i32 = init.trim().parse().unwrap();
    for _ in 0..times {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        x += if input.trim().contains("--") { -1 } else { 1 };
    }
    println!("{x}")
}
