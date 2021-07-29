use std::io::{self, BufRead};

fn perfect_square(n: u32) -> u16 {
    ((n as f64).sqrt() + 0.5) as u16
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.trim().parse().expect("cannot parse input");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read input");

        let n: u32 = input.trim().parse().expect("cannot parse input");

        println!("{}", perfect_square(n));
    }
}
