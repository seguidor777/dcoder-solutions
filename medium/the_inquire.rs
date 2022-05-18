use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // Skip N
    lines.next();
    let mut input = lines.next().unwrap().expect("cannot read people");
    let people: Vec<u8> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    input = lines.next().unwrap().expect("cannot read q");
    let q: u8 = input.parse().expect("cannot parse q");

    for _ in 0..q {
        input = lines.next().unwrap().expect("cannot read queries");
        let queries: Vec<usize> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        let result: u8 = ((&queries[0] - 1)..queries[1]).map(|i| people[i]).sum();

        println!("{}", result);
    }
}
