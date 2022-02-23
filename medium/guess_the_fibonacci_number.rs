use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read n");
    let n: u8 = input.parse().expect("cannot parse n");

    for _ in 0..n {
        input = lines.next().unwrap().expect("cannot read integer");
        let integer: u32 = input.parse().expect("cannot parse integer");
        println!("{}", next_fib(integer));
    }
}

fn next_fib(n: u32) -> u32 {
    let (mut a, mut b) = (0, 1);

    while a + b <= n {
        let aux = a;
        a = b;
        b += aux;
    }

    a + b
}
