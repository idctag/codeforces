use std::io::stdin;
pub fn main() {
    let mut one_pos: (usize, usize) = (0, 0);
    for row_idx in 0..5 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row_values: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .collect();

        for (col_idx, &value) in row_values.iter().enumerate() {
            if value == 1 {
                one_pos = (row_idx, col_idx);
                break;
            }
        }
    }

    let moves = one_pos.0.abs_diff(2) + one_pos.1.abs_diff(2);
    println!("{moves}")
}
