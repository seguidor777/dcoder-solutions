use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let reversed: i32 = input
        .trim()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .expect("cannot parse input");

    println!("{}", reversed);
}
