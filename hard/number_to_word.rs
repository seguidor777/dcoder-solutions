fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let mut current_number = 0;
    let mut total = 0;
    let words: Vec<String> = input.split_whitespace().map(str::to_lowercase).collect();

    for word in words {
        if let Some(number) = to_number(&word) {
            current_number += number;
        } else if &word == "hundred" {
            current_number *= 100;
        } else if &word == "thousand" {
            total += current_number * 1_000;
            current_number = 0;
        } else if &word == "million" {
            total += current_number * 1_000_000;
            current_number = 0;
        } else if &word == "billion" {
            total += current_number * 1_000_000_000;
            current_number = 0;
        } else {
            panic!("invalid word {}", word)
        }
    }

    total += current_number;
    println!("{}", total);
}

fn to_number(word: &str) -> Option<u64> {
    match word {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        "eleven" => Some(11),
        "twelve" => Some(12),
        "thirteen" => Some(13),
        "fourteen" => Some(14),
        "fifteen" => Some(15),
        "sixteen" => Some(16),
        "seventeen" => Some(17),
        "eighteen" => Some(18),
        "nineteen" => Some(19),
        "twenty" => Some(20),
        "thirty" => Some(30),
        "forty" => Some(40),
        "fifty" => Some(50),
        "sixty" => Some(60),
        "seventy" => Some(70),
        "eighty" => Some(80),
        "ninety" => Some(90),
        _ => None,
    }
}
