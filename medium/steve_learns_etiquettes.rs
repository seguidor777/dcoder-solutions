use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("cannot read input");

    println!("{}, please?", &input[..input.len() - 2]);
}
