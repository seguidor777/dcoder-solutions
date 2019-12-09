use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n = input.trim().parse().expect("cannot parse input");

    // Constraints
    if n >= 1 && n <= 26 {
        // Generate string
        let letters = String::from_utf8((b'a'..=b'z').collect()).unwrap();
        let result = &letters[0..n].chars().rev().collect::<String>();

        println!("{}", result);
    }
}
