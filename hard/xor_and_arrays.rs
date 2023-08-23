use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().expect("cannot read line");
    let t: u8 = line.parse().expect("cannot parse t");

    for _ in 0..t {
        line = lines.next().unwrap().expect("cannot read line");
        let mut params = line.split_whitespace();
        let n: usize = params.next().unwrap().parse().expect("cannot parse n");
        let k: u16 = params.next().unwrap().parse().expect("cannot parse k");
        let line = lines.next().unwrap().expect("cannot read line");
        let numbers: Vec<u16> = line
            .split_whitespace()
            .take(n)
            .filter_map(|x| x.parse().ok())
            .collect();
        let partial = numbers.iter().skip(1).fold(numbers[0], |acc, number| acc ^ number);

        println!("{}", partial ^ k);
    }
}
