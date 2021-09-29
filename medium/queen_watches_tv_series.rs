use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.trim().parse().expect("cannot parse input");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read input");
        let bytes = input.as_bytes();

        println!(
            "Season {}, Episode {}",
            10 * (bytes[1] - '0' as u8) + (bytes[2] - '0' as u8),
            10 * (bytes[4] - '0' as u8) + (bytes[5] - '0' as u8)
        );
    }
}
