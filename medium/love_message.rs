use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot parse input");
    let t: u16 = input.parse().expect("cannot parse input");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot parse input");
        let overhead: u16 = input.bytes().map(|b| (b - b'a' + 1) as u16).sum();

        println!("{}", overhead);
    }
}
