use std::io;

pub trait FirstLetterToUppperCase {
    fn first_letter_to_uppper_case(self) -> String;
}

impl FirstLetterToUppperCase for String {
    fn first_letter_to_uppper_case(self) -> String {
        let mut chars = self.chars();

        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    println!(
        "{}",
        input
            .trim()
            .split_whitespace()
            .map(|x| x.to_string().first_letter_to_uppper_case())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
