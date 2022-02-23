use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let v: Vec<usize> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let (_n, k) = (v[0], v[1]);
    input = lines.next().unwrap().expect("cannot read array");
    let mut array: Vec<u16> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    array.sort();

    println!("{}", array.iter().rev().collect::<Vec<_>>()[k - 1]);
}
