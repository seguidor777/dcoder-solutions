use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.parse().expect("cannot parse input");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read input");
        let values: Vec<u32> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        println!("{}", convert_to_base(values[0], values[1]));
    }
}

fn convert_to_base(mut number: u32, base: u32) -> String {
    let mut res = String::new();

    while number > 0 {
        let modulus: u8 = (number % base) as u8;

        if modulus <= 9 {
            res.push((modulus + b'0') as char);
        } else {
            res.push((modulus - 10 + b'A') as char);
        }

        number /= base;
    }

    res.chars().rev().collect()
}
