use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read n");
    let n: u8 = input.parse().expect("cannot parse n");
    let mut valids = 0;

    for _ in 0..n {
        let string = lines.next().unwrap().expect("cannot read string");
        let valid = string.as_bytes().windows(3).all(|w| match *w {
            [open, c, close]
                if (c as char).is_ascii_alphabetic() && (open != b'^' || close != b'^') =>
            {
                false
            }
            _ => true,
        });

        if valid {
            valids += 1;
        }
    }

    println!("{}", valids);
}
