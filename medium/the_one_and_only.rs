use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // Skip n
    lines.next();
    let input = lines
        .next()
        .unwrap()
        .expect("cannot read input");
    let items: Vec<u8> = input
        .split_whitespace()
        .filter_map(|i| i.parse().ok())
        .collect();
    let mut counts: HashMap<u8, u8> = HashMap::new();

    for item in items {
        *counts.entry(item).or_insert(0) += 1
    }

    if let Some((k, _)) = counts.iter().find(|(_, &v)| v == 1) {
        println!("{}", k);
    }
}
