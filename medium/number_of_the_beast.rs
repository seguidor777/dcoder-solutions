
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // Skip n
    lines.next();
    let input = lines.next().unwrap().expect("cannot read input");
    let output: String = input
        .split_whitespace()
        .map(|e| {
            if e != "0" && e.parse::<u16>().expect("cannot parse element") % 6 == 0 {
                "*".repeat(e.len())
            } else {
                e.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", output);
}
