use std::io::{self, BufRead};

fn is_perfect(n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    let mut sum = 1;

    for d in 2..=(n as f64).sqrt() as i32 {
        if n % d == 0 {
            sum += d;

            if n / d != d {
                sum += n / d;
            }
        }
    }

    sum == n
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.trim().parse().expect("cannot parse input");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read input");
        let n: i32 = input.trim().parse().expect("cannot parse input");

        if is_perfect(n) {
            print!("True ");
        } else {
            print!("False ");
        }
    }
}
