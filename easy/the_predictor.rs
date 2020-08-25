use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    if input.contains("000000") || input.contains("111111") {
        println!("Bad");
    } else {
        println!("Good");
    }
}
