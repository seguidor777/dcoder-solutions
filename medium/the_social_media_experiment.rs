use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let capitalized_words: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|s| format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..]))
        .collect();

    println!("{}", capitalized_words.join(" "));
}
