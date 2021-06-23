use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<&str> = input.split_whitespace().collect();

    match v[0].contains(v[1]) {
        true => println!("1"),
        false => println!("0"),
    }
}
