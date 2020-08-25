use std::io;

// Basic Euclidean algorithm for computing the GCD
fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 {
        return y;
    }

    gcd(y % x, x)
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n1: i32 = input.trim().parse().expect("cannot parse input");

    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n2: i32 = input.trim().parse().expect("cannot parse input");
    let result = gcd(n1, n2);

    println!("{}", result);
}
