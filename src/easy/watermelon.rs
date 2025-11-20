use std::io;
pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let w: i32 = input.trim().parse().unwrap();
    if w > 2 && w % 2 == 0 {
        println!("YES")
    } else {
        println!("NO")
    }
}
