use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.trim().parse().expect("cannot parse input");

    for _ in 0..t {
        lines.next().unwrap().expect("cannot read input");
        // Skip n as not needed
        input = lines.next().unwrap().expect("cannot read input");

        let v: Vec<u16> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        let first_diff = v[1] - v[0];
        let second_diff = v[2] - v[1];

        if first_diff == second_diff {
            // First item with a diff different than the first diff (good diff) is wrong
            if let Some((_, wrong)) = &v[3..]
                .iter()
                .enumerate()
                .find(|&(i, n)| n - v[i + 2] != first_diff)
            {
                println!("{}", wrong);
            }

            continue;
        }

        // First item with a diff equals to first or second diff will help to find the wrong one
        let wrong = if v[3] - v[2] == first_diff {
            v[2]
        } else {
            v[1]
        };

        println!("{}", wrong);
    }
}
