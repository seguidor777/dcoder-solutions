use std::io::{self, BufRead};

fn bin_to_dec(bin: &str) -> u16 {
    bin.chars()
        .fold(0, |acc, digit| 2 * acc + digit.to_digit(10).unwrap() as u16)
}

fn is_prime(n: &u16) -> bool {
    // Assumes that 1 is not a prime number
    if *n == 1 {
        return false;
    }

    for i in 2..=((*n as f64).sqrt() as u16) {
        if *n % i == 0 {
            // This means that n has a factor in between 2 and sqrt(n)
            // So it is not a prime number
            return false;
        }
    }

    // If we did not find any factor in the above loop,
    // then n is a prime number
    true
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read input");
    let dec = bin_to_dec(&input);

    println!("{}", dec);

    match is_prime(&dec) {
        true => println!("Prime"),
        false => println!("Not Prime"),
    }
}
