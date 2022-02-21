use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let word = input
        .trim()
        .split_whitespace()
        .map(|s| s.replace(|c: char| !c.is_ascii_alphabetic(), ""))
        .fold(String::new(), |max_word, current_word| {
            match current_word.len() > max_word.len() {
                true => current_word,
                false => max_word,
            }
        });

    println!("{}", word);
}
