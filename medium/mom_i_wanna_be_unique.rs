use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // Skip n
    lines.next();
    let input = lines.next().unwrap().expect("cannot read input");
    let array: Vec<u16> = input
        .split_whitespace()
        .filter_map(|e| e.parse().ok())
        .collect();
    let mut map: HashMap<u16, u8> = HashMap::new();
    array.iter().for_each(|&e| *map.entry(e).or_default() += 1);
    
    if let Some(unique) = map.iter().find(|&(_, &v)| v == 1) {
        println!("{}", unique.0);
    }
}
