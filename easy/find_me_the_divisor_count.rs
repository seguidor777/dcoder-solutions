use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<u32> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let (l, h, d) = (v[0], v[1], v[2]);
    let divisors = (l..=h).filter(|x| x % d == 0).count();

    println!("{}", divisors);
}
