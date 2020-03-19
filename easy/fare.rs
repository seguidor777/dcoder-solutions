use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let (a, m, n, d) = (v[0], v[1], v[2], v[3]);
    let total = a * m + (d - a) * n;

    println!("{}", total.abs());
}
