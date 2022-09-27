use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = &mut stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read t");
    let t: u8 = input.parse().expect("cannot parse t");

    for _ in 0..t {
        let string = lines.next().unwrap().expect("cannot read string");
        let input = lines.next().unwrap().expect("cannot read l");
        let l: usize = input.parse().expect("cannot parse l");
        let dict = &lines
            .map(|line| line.unwrap().trim().to_string())
            .take(l)
            .collect::<HashSet<String>>();

        println!("{}", max_words_count(&string, dict, 0));
    }
}

fn max_words_count(string: &str, dict: &HashSet<String>, initial_count: u8) -> u8 {
    let max_count = dict.iter().fold(initial_count, |mut max_count, word| {
        if string.starts_with(word) {
            let count = max_words_count(string.split_at(word.len()).1, dict, initial_count + 1);

            if count > max_count {
                max_count = count;
            }
        }

        max_count
    });

    if !string.is_empty() && max_count == initial_count {
        return 0;
    }

    max_count
}
