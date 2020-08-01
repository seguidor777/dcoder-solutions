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

    let v: Vec<u32> = input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();
    let average = v.iter().sum::<u32>() as f32 / v.len() as f32;

    // Fix for the round function of Rust, since it rounds from 0.5 up
    if average.fract() < 0.6 {
        println!("{}", average.trunc());
    } else {
        println!("{}", average.round());
    }
}
