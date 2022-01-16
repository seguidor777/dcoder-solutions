use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: u8 = input.trim().parse().expect("cannot parse input");

    for _ in 0..t {
        // Skip n
        lines.next().unwrap().expect("cannot read input");
        input = lines.next().unwrap().expect("cannot read input");
        let mut factors: Vec<u32> = input.split_whitespace().filter_map(|x| x.parse().ok()).collect();
        factors.sort_unstable();
        input = lines.next().unwrap().expect("cannot read input");
        let k: usize = input.trim().parse().expect("cannot parse input");
        let minimum_gifts: u32 = factors.iter().take(k).sum();
 
        println!("{}", minimum_gifts);
    }
}
