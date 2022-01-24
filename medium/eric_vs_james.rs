use std::io;

// Basic Euclidean algorithm for computing the GCD
fn gcd(x: u32, y: u32) -> u32 {
    if x == 0 {
        return y;
    }

    gcd(y % x, x)
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let v: Vec<u8> = input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();
    let smallest: u32 = (v[0]..=v[1]).fold(1, |acc, n| acc * n as u32 / gcd(acc, n as u32));

    println!("{}", smallest);
}
