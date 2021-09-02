use std::io::{self, BufRead};

fn dec_to_bin(mut dec: u16) -> String {
    let mut bin_digits = "".to_string();

    while dec > 0 {
        bin_digits = format!("{}{}", dec % 2, &bin_digits);
        dec /= 2;
    }

    bin_digits
}

fn bin_to_dec(bin: &str) -> u16 {
    bin.chars()
        .fold(0, |acc, digit| 2 * acc + digit.to_digit(10).unwrap() as u16)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.parse().expect("cannot parse input");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read input");

        let n: u16 = input.parse().expect("cannot parse input");
        let reversed_bin: String = dec_to_bin(n).chars().rev().collect();

        println!("{}", bin_to_dec(&reversed_bin));
    }
}
