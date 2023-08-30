use std::convert::TryFrom;

const ONE_BILLION: u64 = 1_000_000_000;
const ONE_MILLION: u64 = 1_000_000;
const ONE_THOUSAND: u64 = 1_000;
const ONES: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const TEENS: [&str; 10] = [
    "",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let mut number: u64 = input.trim().parse().expect("cannot parse number");
    let mut words = Vec::new();

    if number == 0 {
        println!("zero");
        std::process::exit(0);
    }

    if number > ONE_BILLION {
        words.push(format!("{} billion", to_word(usize::try_from(number / ONE_BILLION).unwrap())));
        number %= ONE_BILLION;
    }

    if number > ONE_MILLION {
        words.push(format!("{} million", to_word(usize::try_from(number / ONE_MILLION).unwrap())));
        number %= ONE_MILLION;
    }

    if number > ONE_THOUSAND {
        words.push(format!("{} thousand", to_word(usize::try_from(number / ONE_THOUSAND).unwrap())));
        number %= ONE_THOUSAND;
    }

    if number > 0 {
        words.push(to_word(usize::try_from(number).unwrap()));
    }

    println!("{}", words.join(" "));
}

fn to_word(mut number: usize) -> String {
    let mut words = Vec::new();

    if number >= 100 {
        words.push(format!("{} hundred", ONES[number / 100]));
        number %= 100;
    }

    if number >= 20 {
        words.push(TENS[number / 10].to_string());
        number %= 10;
    } else if number >= 11 {
        words.push(TEENS[number - 10].to_string());
        number = 0;
    }

    if number > 0 {
        words.push(ONES[number].to_string());
    }

    words.join(" ")
}
