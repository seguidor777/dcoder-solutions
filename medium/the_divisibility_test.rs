use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read t");
    let t: u8 = input.parse().expect("cannot parse t");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read integers");
        let v: Vec<u32> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        let (n, x, y) = (v[0], v[1], v[2]);
        let result: Vec<String> = (x..n)
            .filter(|ai| ai % x == 0 && ai % y != 0)
            .map(|ai| ai.to_string())
            .collect();
        println!("{}", result.join(" "));
    }
}
