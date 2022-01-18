use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // Skip n
    lines.next().unwrap().expect("cannot read input");
    let mut input = lines.next().unwrap().expect("cannot read input");
    let integers: Vec<u16> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    input = lines.next().unwrap().expect("cannot read input");
    let q: u16 = input.parse().expect("cannot parse input");

    for _ in 0..q {
        input = lines.next().unwrap().expect("cannot read input");
        let query: u16 = input.parse().expect("cannot parse input");

        match integers.iter().filter(|i| **i == query).count() {
            0 => println!("-1"),
            count => println!("{}", count),
        }
    }
}
