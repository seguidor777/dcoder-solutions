use std::cmp::Ordering;
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: i32 = input.trim().parse().expect("cannot parse input");
    let range: Vec<i32>;

    match n.cmp(&0) {
        Ordering::Less => {
            print!("1.0,");
            range = (n..=-1).rev().collect()
        }
        _ => range = (0..=n).collect(),
    }

    let result = range
        .iter()
        .map(|&e| 2_f32.powi(e).to_string())
        .collect::<Vec<String>>()
        .join(",");

    print!("{}", result);
}
