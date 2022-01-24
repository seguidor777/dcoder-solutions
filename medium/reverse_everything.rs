use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // Skip n
    lines.next().unwrap().expect("cannot read input");
    let input = lines.next().unwrap().expect("cannot read input");
    let mut reversed: String = input
        .split_whitespace()
        .rev()
        .fold(String::new(), |acc, x| acc + x + " ");

    reversed.pop();
    println!("{}", reversed);
}
