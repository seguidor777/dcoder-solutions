use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read t");
    let t: u8 = input.parse().expect("cannot parse t");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read input");
        let chars: String = input
            .chars()
            .enumerate()
            .step_by(2)
            .take_while(|&(i, _)| i < input.len() / 2)
            .map(|(_, c)| c)
            .collect();
        println!("{}", chars);
    }
}
