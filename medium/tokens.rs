use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.parse().expect("cannot parse input");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read input");
        let v: Vec<&str> = input.split_whitespace().collect();
        let token = v.get(1).ok_or(()).expect("cannot parse input");
        let (mut has_letter, mut has_digit) = (false, false);

        for c in token.chars() {
            if c.is_ascii_alphabetic() { has_letter = true  }
            if c.is_ascii_digit() { has_digit = true }
            if has_letter && has_digit { break }
        }

        match (has_letter, has_digit) {
            (true, true) => println!("Variable"),
            (true, false) => println!("String"),
            (false, true) => println!("Number"),
            _ => ()
        }
    }
}
