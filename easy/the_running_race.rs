use std::cmp::Ordering;
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
    let (_d, x, y) = (v[0], v[1], v[2]);

    match x.cmp(&y) {
        Ordering::Less => println!("Ryan"),
        Ordering::Greater => println!("Alex"),
        Ordering::Equal => println!("Draw"),
    }
}
