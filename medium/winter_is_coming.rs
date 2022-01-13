use std::io::{self, BufRead};

fn sum_alternates<'a, I>(soldiers: I) -> u16
where
    I: Iterator<Item = &'a u16>,
{
    soldiers.enumerate().filter(|(i, _)| i % 2 == 0).fold(0, |acc, (_, s)| acc + s)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.parse().expect("cannot parse input");

    for _ in 0..t {
        lines.next();
        input = lines.next().unwrap().expect("cannot read input");

        let soldiers: Vec<u16> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        let maximum = std::cmp::max(sum_alternates(soldiers.iter()), sum_alternates(soldiers.iter().skip(1)));

        println!("{}", maximum);
    }
}
