use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let v: Vec<usize> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let d = v[1];
    input = lines.next().unwrap().expect("cannot read input");
    let mut array: Vec<&str> = input.split_whitespace().collect();

    array.rotate_left(d);
    println!("{}", array.join(" "))
}
