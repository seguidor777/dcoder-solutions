use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let mut line = input.split_whitespace();
    let _n = line.next();
    let k: usize = line.next().unwrap().parse().expect("cannot parse input");
    let array_input = lines.next().unwrap().expect("cannot read input");
    let mut array: Vec<&str> = array_input.split_whitespace().collect();

    array.rotate_right(k);
    println!("{}", array.join(" "))
}
