use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let garry_points: Vec<u32> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let sharry_points: Vec<u32> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let x: u32 = garry_points.iter().sum();

    match sharry_points.iter().sum::<u32>() {
        y if x > y => println!("Garry {}", x - y),
        y if y > x => println!("Sharry {}", y - x),
        y if x == y => println!("None 0"),
        _ => (),
    }
}
