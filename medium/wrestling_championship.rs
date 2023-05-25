use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: u32 = lines.next().unwrap().parse().unwrap();

        println!("{}", n - 1);
    }
}
