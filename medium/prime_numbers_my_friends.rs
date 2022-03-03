use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("cannot read n");
    let n: usize = input.trim().parse().expect("cannot parse n");
    let primes: Vec<usize> = sieve(n);
    let first_half: Vec<&usize> = primes.iter().take_while(|&&p| p <= n / 2).collect();
    let pairs: Vec<(usize, usize)> = first_half
        .iter()
        .flat_map(|&&a| {
            primes[(first_half.len() - 1)..primes.len()]
                .iter()
                .map(move |&b| (a, b))
        })
        .filter(|&(a, b)| a + b == n)
        .collect();

    if pairs.is_empty() {
        println!("No");
        return;
    }

    for (a, b) in pairs {
        println!("{} {}", a, b);
    }
}

fn sieve(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 {
        is_prime[1] = false
    }

    for n in 2..((limit as f64).sqrt() as usize + 1) {
        if is_prime[n] {
            let mut multiple = n * n;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += n;
            }
        }
    }

    is_prime
        .into_iter()
        .enumerate()
        .filter_map(|(i, item)| if item { Some(i) } else { None })
        .collect()
}
