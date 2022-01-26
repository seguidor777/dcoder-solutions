use std::io;

const DECIMAL_BASE: u32 = 10;
const BINARY_BASE: u32 = 2;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let complement = binary_to_decimal(&input.trim()) + 1;

    println!("{}", complement);
}

fn binary_to_decimal(binary: &str) -> u32 {
    let mut decimal = 0;
    let mut base = 1;

    for mut digit in binary.chars().rev() {
        digit = if digit == '0' { '1' } else { '0' };
        decimal += digit.to_digit(DECIMAL_BASE).unwrap() * base;
        base *= BINARY_BASE;
    }

    decimal
}
