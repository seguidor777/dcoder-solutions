use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let values: Vec<u8> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let (_n, desired_sum) = (values[0], values[1]);
    input = lines.next().unwrap().expect("cannot read input");
    let array: Vec<u8> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    match array
        .iter()
        .enumerate()
        .any(|(i, a)| array[i..].iter().any(|b| a + b == desired_sum))
    {
        true => println!("Yes"),
        false => println!("No"),
    }
}
