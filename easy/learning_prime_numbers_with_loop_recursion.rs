use std::io;

fn is_prime(n: &i32) -> bool {
    // Assumes that 1 is not a prime number
    if *n == 1 {
        return false;
    }

    for i in 2..=((*n as f64).sqrt() as i32) {
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
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    for n in v[0]..=v[1] {
        if is_prime(&n) {
            println!("{}", n)
        }
    }
}
