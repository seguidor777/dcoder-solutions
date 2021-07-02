use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<u8> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    match v[0] < v[1] {
        true => match v[1] < v[2] {
            true => println!("{} {} {}", v[0], v[1], v[2]),
            false => match v[0] < v[2] {
                true => println!("{} {} {}", v[0], v[2], v[1]),
                false => println!("{} {} {}", v[2], v[0], v[1]),
            },
        },
        false => match v[0] > v[2] {
            true => match v[1] < v[2] {
                true => println!("{} {} {}", v[1], v[2], v[0]),
	        false => println!("{} {} {}", v[2], v[1], v[0]),
            },
            false => println!("{} {} {}", v[1], v[0], v[2]),
        },
    }
}
