use std::io::{self, BufRead};

fn is_power(n: u64) -> bool {
    let mut pow: u64 = 1;
    let mut result: u64 = 0;

    while result < n {
        result = pow.pow(pow as u32);
        pow += 1;
    }

    result == n
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let n: u8 = input.trim().parse().expect("cannot parse input");

    input = lines.next().unwrap().expect("cannot read input");

    let integers: Vec<u64> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let mut answers: Vec<&str> = Vec::new();

    for i in 0..n {
        match is_power(integers[i as usize]) {
            true => answers.push("Yes"),
            false => answers.push("No"),
        }
    }

    println!("{}", answers.join(" "));
}
