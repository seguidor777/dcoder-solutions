use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.parse().expect("cannot parse n");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read input");
        let v: Vec<i32> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        let k = v[1];
        input = lines.next().unwrap().expect("cannot read input");
        let array: Vec<i32> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        let count = array.iter().filter(|&x| *x * 2 == k).count();

        println!("{}", count);
    }
}
