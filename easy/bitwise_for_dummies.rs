use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<i16> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let (a, b, p, q, x) = (v[0], v[1], v[2], v[3], v[4]);

    println!("{}", a & b);
    println!("{}", a | b);
    println!("{}", a ^ b);
    println!("{}", p << q);
    println!("{}", p >> q);
    println!("{}", !x);
}
