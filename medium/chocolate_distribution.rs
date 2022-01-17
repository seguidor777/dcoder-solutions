use std::io::{self, BufRead};

fn distribute_chocolates(m: u32, n: u32, friends: &mut Vec<u32>) {
    let (div, rem) = (m / n, m % n);

    for _ in 0..n {
        friends.push(div);
    }

    for i in 0..rem as usize {
        friends[i] += 1;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.parse().expect("cannot parse input");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read input");
        let numbers: Vec<u32> = input
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
        let (n, m) = (numbers[0] + 1, numbers[1]);
        let mut friends: Vec<u32> = Vec::with_capacity(n as usize);

        distribute_chocolates(m, n, &mut friends);

        println!(
            "{}",
            friends
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
