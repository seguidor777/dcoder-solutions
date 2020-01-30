use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let c = input.trim().chars().next().expect("cannot parse input");
    let c_ascii = c as u8;

    // Constraints
    if c_ascii >= 33 && c_ascii <= 127 {
        println!("{}", c_ascii)
    }
}
