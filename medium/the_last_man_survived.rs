use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read T");
    let t: u8 = input.parse().expect("cannot parse T");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read N");
        let n: u16 = input.parse().expect("cannot parse N");
        let mut m = 1;

        while m <= n {
            m *= 2
        }

        println!("{}", 2 * n - m + 1);
    }
}
