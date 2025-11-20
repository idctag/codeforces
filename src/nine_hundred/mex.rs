use std::collections::HashSet;
use std::io::{self, BufRead};

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t_line = lines.next().unwrap().unwrap();
    let t: i32 = t_line.trim().parse().unwrap();
    for _ in 0..t {
        let nk_line = lines.next().unwrap().unwrap();
        let nk_values: Vec<i32> = nk_line
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();

        let k = nk_values[1];

        let nums_input = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = nums_input
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();

        let mut unique_nums: HashSet<i32> = HashSet::new();
        let mut count_k = 0;
        for &n in &nums {
            unique_nums.insert(n);
            if n == k {
                count_k += 1
            }
        }
        let mut missing_m = 0;
        for i in 0..k {
            if !unique_nums.contains(&i) {
                missing_m += 1
            }
        }

        let min_operations = missing_m.max(count_k);
        println!("{}", min_operations)
    }
}
