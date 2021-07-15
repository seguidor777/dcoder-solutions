use std::io;

// Function to find multiple
fn find_multiple(x: u64, y: u64) -> u64 {
    for m in 1..y {
        if x * m % y == 0 {
            return m;
        }
    }

    y
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: u8 = input.trim().parse().expect("cannot parse input");

    for _ in 0..n {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("cannot read input");

        let v: Vec<u64> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        let multiple = find_multiple(v[0], v[1]);

        println!("{}", multiple)
    }
}
