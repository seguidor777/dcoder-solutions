use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // Skip n
    lines.next().unwrap().expect("cannot read input");
    let mut input = lines.next().unwrap().expect("cannot read input");
    let rose: Vec<i16> = input.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    input = lines.next().unwrap().expect("cannot read input");
    let jack: Vec<i16> = input.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    let mut merged = rose;

    merged.extend(jack);
    merged.sort();
    println!("{}", merged.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
