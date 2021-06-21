use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let result = input.split_whitespace().fold(0, |acc, bit| {
        acc ^ bit.parse::<u8>().expect("cannot parse bit")
    });

    print!("{}", result);
}
