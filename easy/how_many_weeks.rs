use std::io;

const TANK_CAPACITY: u32 = 10000;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let weekly_usage: u32 = input.trim().parse().expect("cannot parse input");

    println!("{}", TANK_CAPACITY / weekly_usage)
}
