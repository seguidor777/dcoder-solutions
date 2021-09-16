use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let n: u8 = input.trim().parse().expect("cannot parse input");
    let mut dictionary: HashMap<String, u8> = HashMap::with_capacity(n as usize);

    for _ in 0..n {
        input = lines.next().unwrap().expect("cannot read input");
        let pair: Vec<_> = input.split_whitespace().collect();
        let (key, value): (String, u8) = (
            pair[0].to_string(),
            pair[1].parse().expect("cannot parse number"),
        );
        dictionary.insert(key, value);
    }

    input = lines.next().unwrap().expect("cannot read input");
    let q: u8 = input.trim().parse().expect("cannot parse input");

    for _ in 0..q {
        let key = lines.next().unwrap().expect("cannot read input");
        println!("{}", dictionary[&key]);
    }
}
