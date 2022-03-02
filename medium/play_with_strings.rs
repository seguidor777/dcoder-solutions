use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read n");
    let n: u8 = input.trim().parse().expect("cannot parse n");

    for _ in 0..n {
        input = lines.next().unwrap().expect("cannot read words");
        let words: Vec<String> = input
            .split_whitespace()
            .map(|w| w.chars().rev().collect())
            .collect();

        println!("{}", words.join(" "));
    }
}
