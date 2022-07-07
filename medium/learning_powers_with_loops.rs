use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap().expect("cannot read line");
    let v: Vec<u64> = line.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    let (m, n) = (v[0], v[1]);
    let mut r: u64 = 1;

    for _ in 0..n {
        r *= m;
    }

    println!("{}", r);
}
