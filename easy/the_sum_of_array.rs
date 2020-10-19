use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let _n: u32 = input.trim().parse().expect("cannot parse input");

    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let numbers: Vec<u32> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let sum: u32 = numbers.iter().sum();

    println!("{}", sum);
}
