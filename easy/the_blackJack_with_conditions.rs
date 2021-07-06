use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<u8> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let sum: u8 = v
        .iter()
        .map(|&x| match x == 11 {
            true => 1,
            false => x,
        })
        .sum();

    match sum <= 21 {
        true => println!("{}", sum),
        false => println!("0"),
    }
}
