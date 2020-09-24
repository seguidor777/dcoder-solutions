use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    match is_palindrome(&input.trim().to_string()) {
        true => println!("Yes"),
        false => println!("No"),
    }
}

// Check if a word is a palindrome
fn is_palindrome(word: &String) -> bool {
    let rev_word: String = word.chars().rev().collect();
    let middle_index = word.len() / 2;

    word[0..middle_index] == rev_word[0..middle_index]
}
