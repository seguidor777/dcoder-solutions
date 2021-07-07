use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<i8> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let a_distance = v[0].pow(2) + v[1].pow(2);
    let b_distance = v[2].pow(2) + v[3].pow(2);

    match a_distance < b_distance {
        true => println!("A"),
        false => println!("B"),
    }
}
