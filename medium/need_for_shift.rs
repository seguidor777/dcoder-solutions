use std::char;
use std::io::{self, BufRead};

const FIRST_CHAR_CODE: u8 = 'a' as u8;
const LAST_CHAR_CODE: u8 = 'z' as u8;
const CHARS_TOTAL: u8 = LAST_CHAR_CODE - FIRST_CHAR_CODE + 1;

fn shift_word(word: &str, d: u8) -> String {
    let d = d % CHARS_TOTAL;

    word.chars()
        .map(|c| {
            let mut char_code = c as u8 - d;

            if char_code < FIRST_CHAR_CODE {
                char_code += CHARS_TOTAL;
            }

            char_code as char
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let word = lines.next().unwrap().expect("cannot read input");
    let input = lines.next().unwrap().expect("cannot read input");
    let d: u8 = input.parse().expect("cannot parse input");

    println!("{}", shift_word(&word, d));
}
