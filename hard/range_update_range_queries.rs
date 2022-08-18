use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read input");
    let v: Vec<u16> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let Q = v[1];
    let input = lines.next().unwrap().expect("cannot read array");
    let mut A: Vec<u8> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    for _ in 0..Q {
        let input = lines.next().unwrap().expect("cannot read query");
        let v: Vec<usize> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        let (q, l, r) = (v[0], v[1], v[2]);

        if q == 1 {
            println!("{}", A[l - 1..r].iter().min().unwrap());
        } else if q == 2 {
            let p = v[3];

            A[l - 1..r].iter_mut().for_each(|x| *x += p as u8);
        }
    }
}
