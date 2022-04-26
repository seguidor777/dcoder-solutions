use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let glin_string = lines.next().unwrap().expect("cannot read Glin's string");
    let cody_string = lines.next().unwrap().expect("cannot read Cody's string");
    let mut glin_chars: Vec<char> = glin_string.chars().collect();
    let mut cody_chars: Vec<char> = cody_string.chars().collect();

    glin_chars.sort_by(|a, b| b.cmp(a));
    cody_chars.sort_by(|a, b| b.cmp(a));

    if glin_chars == cody_chars {
        println!("Yes");
    } else {
        println!("No");
    }
}
