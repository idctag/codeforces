use std::io::stdin;

pub fn main() {
    let times = input_num();
    let mut scores: Vec<i64> = Vec::new();
    for _ in 0..times {
        let (req_amount, minutes) = input_two();
        let mut reqs = get_requirements(req_amount);

        reqs.insert(0, (0,0));
        let mut max_scores: i64 = 0;
        for i in 0..(reqs.len() - 1) {
            let (t0, s0) = reqs[i];
            let (t1, s1) = reqs[i + 1];
            let time_span = t1 - t0;

            if time_span == 0 {continue;}

            let required_parity = if s0 == s1 {0} else {1};

            let time_parity = time_span % 2;
            if required_parity == time_parity {
                max_scores += time_span
            } else {
                max_scores += time_span -1
            }
        }
        if let Some(&(t_last, _)) = reqs.last() {
            let time_span_final = minutes - t_last;
            if time_span_final > 0 {
                max_scores += time_span_final
            }
        }
        scores.push(max_scores);
    }
    for s in scores{
        println!("{s}")
    }
} 

fn get_requirements(n: i64) -> Vec<(i64, i64)> {
    let mut res: Vec<(i64, i64)> = Vec::new();
    for _ in 0..n {
        res.push(input_two());
    }
    res
}


fn input_num() -> i64 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let val = input.trim().parse::<i64>().unwrap();
    val
}

fn input_two() -> (i64, i64) {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let nums: Vec<i64> =input.trim().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
    (nums[0], nums[1])
}
