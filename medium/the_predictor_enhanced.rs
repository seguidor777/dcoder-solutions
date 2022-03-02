use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read number");
    let number: u32 = input.trim().parse().expect("cannot parse number");
    let binary = format!("{:b}", number);

    match binary.contains("111111") || binary.contains("000000") {
        true => println!("Bad"),
        false => println!("Good"),
    }
}
