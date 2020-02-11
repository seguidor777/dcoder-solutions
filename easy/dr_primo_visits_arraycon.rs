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
    return true;
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let _n: i32 = input.trim().parse().expect("cannot parse input");

    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let a: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let mut primes = 0;

    for n in a.iter() {
        if is_prime(n) {
            primes += 1;
        }
    }

    println!("{}", primes);
}
