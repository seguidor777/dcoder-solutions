use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read T");
    let t: u8 = input.parse().expect("cannot parse T");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read T");
        let n: u16 = input.parse().expect("cannot parse N");
        let n_sqrt = (n as f64).sqrt();
        let n_sqrt_floor = n_sqrt.floor() as u16;

        if n_sqrt == n_sqrt_floor.into() {
            println!("0");
            continue;
        }

        let mut i = n_sqrt_floor;
        let min = (n - n_sqrt_floor.pow(2)).min((n_sqrt_floor + 1).pow(2) - n);

        println!("{}", min);
    }
}
