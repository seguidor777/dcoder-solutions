use std::io;

// Basic Euclidean algorithm for computing the GCD
fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 {
        return y;
    }

    return gcd(y % x, x);
}

// Reduce a fraction to its lowest form
fn reduce_fraction(mut n: i32, mut d: i32) -> (i32, i32) {
    let tmp = gcd(n, d);

    n /= tmp;
    d /= tmp;

    return (n, d);
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

    let (n, d) = (v[0], v[1]);

    // Constraints
    if n >= 1 && n <= 1000 && d >= 1 && d <= 1000 {
        let (min_n, min_d) = reduce_fraction(n, d);

        println!("{} {}", min_n, min_d);
    }
}
