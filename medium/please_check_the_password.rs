use std::io::{self, BufRead};

fn find_password(words: &[String]) -> Option<&str> {
    for (i, w) in words.iter().enumerate() {
        for w2 in &words[i + 1..] {
            if *w == w2.chars().rev().collect::<String>() {
                return Some(w);
            }
        }
    }

    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read input");
    let n: u8 = input.parse().expect("cannot parse input");
    let mut words: Vec<String> = Vec::new();

    for _ in 0..n {
        let word = lines.next().unwrap().expect("cannot read input");

        words.push(word);
    }

    if let Some(password) = find_password(&words) {
        let password_len: usize = password.len();
        let middle = password.chars().nth(password_len / 2).unwrap();

        println!("{} {}", password_len, middle);
    }
}
