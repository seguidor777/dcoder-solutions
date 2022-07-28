use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.trim().parse().expect("cannot parse input");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read input");
        
        let evens: String = input.chars().step_by(2).collect();
        let odds: String = input.chars().skip(1).step_by(2).collect();

        println!("{} {}", evens, odds);
    }
}
