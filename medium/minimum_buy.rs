use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input: String = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.parse().expect("cannot parse input");

    for _ in 0..t {
        let fruits = lines.next().unwrap().expect("cannot read input");
        let mut map = HashMap::new();

        for c in fruits.chars() {
            *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        }

        let mut total_cost: u16 = 0;

        for &value in map.values() {
            total_cost += value / 2;

            if value % 2 == 1 {
                total_cost += 1;
            }
        }

        println!("{}", total_cost);
    }
}
