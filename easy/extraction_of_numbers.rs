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
    let mut digits = vec![];

    // Constraints
    if n >= 1 && n <= 50 {
        // Extract all digits in each word
        for word in words.iter() {
            for c in word.chars() {
                if c.is_digit(10) {
                    digits.push(c.to_string())
                }
            }
        }
    }

    println!("{}", digits.join(" "));
}
