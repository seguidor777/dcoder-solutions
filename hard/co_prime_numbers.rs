use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().expect("cannot read line");
    let x: u16 = line.parse().expect("cannot parse x");
    line = lines.next().unwrap().expect("cannot read line");
    let n: usize = line.parse().expect("cannot parse n");
    line = lines.next().unwrap().expect("cannot read line");
    let lands = line
        .split_whitespace()
        .take(n)
        .filter(|p| {
            let people: u16 = p.parse().expect("cannot parse people");

            gcd(x, people) == 1
        })
        .count();

    println!("{}", lands);
}

fn gcd(x: u16, y: u16) -> u16 {
    if x == 0 {
        return y;
    }

    gcd(y % x, x)
}
