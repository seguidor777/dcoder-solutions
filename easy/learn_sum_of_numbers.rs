use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let sum = input.split_whitespace().fold(0, |sum, n| {
        sum + n.parse::<u8>().expect("cannot parse number")
    });

    print!("{}", sum);
}
