use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap().expect("cannot read line");
    let mut params = line.split_whitespace();
    let _n: usize = params.next().unwrap().parse().expect("cannot parse n");
    let q: usize = params.next().unwrap().parse().expect("cannot parse q");
    let string = lines.next().unwrap().expect("cannot read string");
    let chars_array: Vec<char> = string.chars().collect();

    for _ in 0..q {
        let line = lines.next().unwrap().expect("cannot read line");
        let mut params = line.split_whitespace();
        let start: usize = params.next().unwrap().parse().expect("cannot parse n");
        let end: usize = params.next().unwrap().parse().expect("cannot parse q");
        let smallest: char = *chars_array[start - 1..end].iter().min().unwrap();

        println!("{}", smallest);
    }
}
