use std::io;

fn is_prime(n: &u32) -> bool {
    // Assumes that 1 is not a prime number
    if *n == 1 {
        return false;
    }

    for i in 2..=((*n as f64).sqrt() as u32) {
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
    io::stdin().read_line(&mut input).expect("cannot read input");
    let v: Vec<u32> = input.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    let (l, r) = (v[0], v[1]);
    let primes: Vec<u32> = (l..=r).filter(|&n| is_prime(&n)).collect();
    println!("{}", primes.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" "));
}
