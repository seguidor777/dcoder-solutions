use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.parse().expect("cannot parse T");

    for _ in 0..t {
        let word = lines.next().unwrap().expect("cannot read input");
        let output = match word.as_bytes() {
            [start, b'e', b'r'] => format!("{}{}", start, "est"),
            [.., b'e', b's', b't'] => word,
            [start, b'e'] => format!("{}{}", start, "r"),
            _ => format!("{}{}", word, "er"),
        };

        println!("{}", output);
    }
}
