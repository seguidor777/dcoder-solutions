use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: usize = input.trim().parse().expect("cannot parse input");

    println!("Dc{}der", "o".repeat(n));
}
