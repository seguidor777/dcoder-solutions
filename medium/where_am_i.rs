use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let q = lines.next().unwrap().expect("cannot read input");
    let p = lines.next().unwrap().expect("cannot read input");

    match q.find(&p) {
        Some(index) => println!("{}", index),
        None => println!("-1"),
    }
}
