use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap().expect("cannot read line");
    let mut params = line.split_whitespace();
    let n: usize = params.next().unwrap().parse().expect("cannot parse n");
    let m: usize = params.next().unwrap().parse().expect("cannot parse m");
    let mut haunted_columns = HashSet::new();
    let mut total_cost = 0;

    for line in lines.take(n) {
        for (j, x) in line.unwrap().split_whitespace().take(m).enumerate() {
            let cost: u8 = x.parse().expect("cannot parse cost");

            if haunted_columns.get(&j).is_none() {
                if cost == 0 {
                    haunted_columns.insert(j);
                } else if cost > 0 {
                    total_cost += cost
                }
            }
        }
    }

    println!("{}", total_cost);
}
