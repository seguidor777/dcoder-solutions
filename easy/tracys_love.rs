use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let (a, b) = (v[0], v[1]);

    if a + b == 6 || (a - b).abs() == 6 {
        println!("Love");
    } else {
        println!("Hate");
    }
}
