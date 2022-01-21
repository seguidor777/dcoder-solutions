use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let words: Vec<&str> = input.chars().map(digit_to_word).collect();

    println!("{}", words.join(" "));
}

fn digit_to_word(d: char) -> &'static str {
    match d {
        '1' => "one",
        '2' => "two",
        '3' => "three",
        '4' => "four",
        '5' => "five",
        '6' => "six",
        '7' => "seven",
        '8' => "eight",
        '9' => "nine",
        '0' => "zero",
        _ => "",
    }
}
