use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap().expect("cannot read input");
    let n: usize = line.parse().expect("cannot parse N");
    let (mut l_shoes, mut r_shoes) = (Vec::with_capacity(n), HashMap::new());

    for l in lines.take(n) {
        let line = l.unwrap();
        let mut params = line.split_whitespace();
        let size: u8 = params.next().unwrap().parse().expect("cannot parse size");
        let side: char = params.next().unwrap().parse().expect("cannot parse side");

        match side {
            'L' => l_shoes.push(size),
            'R' => {
                r_shoes
                    .entry(size)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
            _ => panic!("invalid side"),
        }
    }

    let pairs = l_shoes.iter().fold(0, |pairs, l_shoe| {
        if let Some(r_shoe) = r_shoes.get_mut(l_shoe) {
            if *r_shoe > 0 {
                *r_shoe -= 1;

                return pairs + 1;
            }
        }

        pairs
    });

    println!("{}", pairs);
}
