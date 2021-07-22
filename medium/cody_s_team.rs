use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let mut line = input.split_whitespace();
    let n: u16 = line.next().unwrap().parse().unwrap();
    let k: u16 = line.next().unwrap().parse().unwrap();
    let input = lines.next().unwrap().expect("cannot read input");
    let v: Vec<i16> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    if let Some(highest) = (0..=n - k)
        .map(|i| (i..i + k).fold(0, |acc, j| acc + v[j as usize]))
        .max() {
        println!("{}", highest);
    }
}
