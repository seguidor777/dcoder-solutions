use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read T");
    let t: u8 = input.parse().expect("cannot parse T");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read size");
        let size: usize = input.parse().expect("cannot parse size");
        input = lines.next().unwrap().expect("cannot read S");
        let s: Vec<u8> = input
            .split_whitespace()
            .take(size)
            .filter_map(|x| x.parse().ok())
            .collect();
        let result = xorall(&s);
        println!("{:?}", result);
    }
}

fn xorall(set: &[u8]) -> u8 {
    let n = set.len();
    let mut subsets: Vec<Vec<u8>> = Vec::new();

    // Iterate through all possible binary representations of integers from 0 to 2^n - 1
    for i in 0..(1 << n) {
        let mut subset: Vec<u8> = Vec::new();

        // Construct the subset based on the binary representation
        for (j, &item) in set.iter().enumerate().take(n) {
            if (i & (1 << j)) != 0 {
                subset.push(item);
            }
        }

        if !subset.is_empty() {
            subsets.push(subset)
        }
    }

    subsets
        .iter()
        .fold(0, |acc, x| acc ^ x.iter().fold(0, |acc, x| acc ^ x))
}
