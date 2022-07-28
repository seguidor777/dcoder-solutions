use std::io;

const VOWELS: &[&str] = &["a", "e", "i", "o", "u"];

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("cannot read input");

    let word = input.trim();

    if VOWELS.iter().any(|&v| word.starts_with(v)) {
        println!("{}way", word);
    } else {
        println!("{}{}ay", &word[1..], &word[0..1]);
    }
}
