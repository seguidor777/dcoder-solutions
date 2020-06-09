use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    for c in input.trim().chars() {
        print!("{}", (c as u8 + 1) as char);
    }
}
