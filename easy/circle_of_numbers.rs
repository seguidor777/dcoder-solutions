use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let (n, x) = (v[0], v[1]);

    // Constraints
    if n >= 4 && n <= 20 && x >= 0 && x < n {
        // Determine dancing partner of X
        if n % 2 == 0 {
            let partner;
            let half = n / 2;

            if x < half {
                partner = x + half;
            } else {
                partner = x - half;
            }

            println!("{}", partner);
        }
    }
}
