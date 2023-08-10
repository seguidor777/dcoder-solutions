use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.parse().expect("cannot parse T");

    for _ in 0..t {
        let word = lines.next().unwrap().expect("cannot read input");
        let mut even = String::new();
        let mut odd = String::new();

        for (i, b) in word.as_bytes().iter().enumerate() {
            if i % 2 == 0 {
                even.push(*b as char);
            } else {
                odd.push(*b as char);
            }
        }

        println!("{} {}", even, odd);
    }
}
