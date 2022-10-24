use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read T");
    let t: u8 = input.parse().expect("cannot parse T");

    for _ in 0..t {
        let input = lines.next().unwrap().expect("cannot read N");
        let n: u32 = input.parse().expect("cannot parse N");
        let squares: u32 = (1..=n).map(|x| x * x).sum();

        println!("{}", squares);
    }
}
