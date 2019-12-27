use std::collections::HashMap;
use std::io;

const ALPHABET_SIZE: usize = 26;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let mut chars = HashMap::new();

    // Constraints
    if input.len() >= 1 && input.len() <= 100 {
        // Find if the string is a pangram
        for c in input.chars() {
            if c.is_alphabetic() {
                chars.insert(c.to_lowercase().to_string(), true);
            }
        }

        if chars.len() == ALPHABET_SIZE {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
