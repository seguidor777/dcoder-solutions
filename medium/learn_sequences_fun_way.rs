use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // Skip n
    lines.next();
    let input = lines.next().unwrap().expect("cannot read integers");
    let integers: Vec<i8> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let mut integers_iter = integers.windows(2);
    let first_part: Vec<&[i8]> = integers_iter.by_ref().take_while(|w| w[0] > w[1]).collect();
    let second_part: Vec<&[i8]> =  integers_iter.by_ref().take_while(|w| w[0] < w[1]).collect();

    if first_part.len() > 0 && second_part.len() > 0 {
        println!("Yes");
        return;
    }

    println!("No");
}
