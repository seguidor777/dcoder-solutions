use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let n: u16 = input.trim().parse().expect("cannot parse input");
    let mut zeroes = 0;
    let mut divisor = 5;

    while n / divisor >= 1 {
        zeroes += n / divisor;
        divisor *= 5;
    }

    println!("{}", zeroes);
}
