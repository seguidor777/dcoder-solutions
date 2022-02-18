use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    input = input.trim().to_string();
    let compressed = compress(&input);

    if compressed.len() < input.len() {
        println!("{}", compressed);
    } else {
        println!("{}", input);
    }
}

pub fn compress(input: &str) -> String {
    let mut compressed = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        let mut count: u8 = 1;

        while Some(&c) == chars.peek() {
            count += 1;
            chars.next();
        }

        match count {
            1 => compressed.push(c),
            _ => compressed.push_str(&format!("{}{}", c, count)),
        }
    }

    compressed
}
