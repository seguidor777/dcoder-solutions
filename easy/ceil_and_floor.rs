use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("cannot read x");

    let x: f32 = input.trim().parse().expect("cannot parse x");

    println!("{} {}", x.ceil(), x.floor());
}
