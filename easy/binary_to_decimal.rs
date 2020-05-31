use std::io;

const DECIMAL_BASE: u32 = 10;
const BINARY_BASE: u32 = 2;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let decimal = binary_to_decimal(&input.trim());

    print!("{}", decimal);
}

fn binary_to_decimal(binary: &str) -> u32 {
    let mut decimal = 0;
    let mut base = 1;

    for digit in binary.chars().rev() {
        decimal += digit.to_digit(DECIMAL_BASE).unwrap() * base;
        base *= BINARY_BASE;
    }

    decimal
}
