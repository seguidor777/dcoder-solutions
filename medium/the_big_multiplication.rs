use std::io::{self, BufRead};

const STORAGE_LIMIT: u64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // Skip N
    lines.next();
    let input = lines.next().unwrap().expect("cannot read array items");
    let items: Vec<u64> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let res = items
        .iter()
        .fold(1, |acc, &x| acc * x % STORAGE_LIMIT);

    println!("{}", res);
}
