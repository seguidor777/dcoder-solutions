use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: u32 = input.trim().parse().expect("cannot parse input");

    input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let words: Vec<&str> = input.split_whitespace().collect();

    // Constraints
    if n >= 1 && n <= 1000 {
        for word in words.iter() {
            if !(word.len() >= 1 && word.len() <= 50) {
                return;
            }
        }

        match words.iter().position(|&w| w == "Nemo") {
            Some(position) => println!("{}", position + 1),
            None => {}
        }
    }
}
