use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot parse input");

    let mut ocurrencies = HashMap::new();

    for c in input.trim().chars() {
        if !ocurrencies.contains_key(&c) {
            print!("{}", c);
            ocurrencies.insert(c, true);
        }
    }
}
