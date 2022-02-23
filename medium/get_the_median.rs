use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // Skip n
    lines.next();
    let input = lines.next().unwrap().expect("cannot read array");
    let mut array: Vec<u16> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let array_len = array.len();
    array.sort_unstable();

    let median = if array_len & 1 == 0 {
        array[array_len / 2 - 1]
    } else {
        array[array_len / 2]
    };

    println!("{}", median);
}
