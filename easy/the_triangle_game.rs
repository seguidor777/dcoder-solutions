use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    input = input.trim().to_string();

    for i in 1..=input.len() {
        println!("{}", &input[..i]);
    }
}
