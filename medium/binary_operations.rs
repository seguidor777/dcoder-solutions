use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let numbers: Vec<&str> = input.split_whitespace().collect();
    let numbers_rev: Vec<String> = numbers.iter().map(|n| n.chars().rev().collect()).collect();
    let max_len = numbers_rev.iter().map(|n| n.len()).max().unwrap();

    println!("{}", binary_sum(&numbers_rev, max_len));
    println!("{}", binary_product((&numbers_rev[0], &numbers_rev[1]), max_len));
}

fn binary_sum(numbers: &Vec<String>, max_len: usize) -> String {
    let mut carry = 0;
    let mut result = String::new();

    for i in 0..max_len {
        let mut col_sum = carry;

        for number in numbers {
            if let Some(digit) = number.chars().nth(i) {
                if digit == '1' {
                    col_sum += 1
                }
            }
        }

        carry = col_sum / 2;
        result.push_str(&(col_sum % 2).to_string());
    }

    while carry > 1 {
        carry /= 2;
        result.push_str(&(carry % 2).to_string());
    }

    if carry == 1 {
        result.push('1');
    }

    result.chars().rev().collect()
}

fn binary_product(numbers: (&str, &str), max_len: usize) -> String {
    let mut summands: Vec<String> = Vec::new();

    let (number_a, number_b) = match numbers.1.len() < max_len {
        true => (numbers.1, numbers.0),
        false => numbers,
    };

    for (i, a) in number_a.chars().enumerate() {
        let mut summand = "0".repeat(i);

        for b in number_b.chars() {
            let r = a.to_digit(10).unwrap() * b.to_digit(10).unwrap();

            summand.push_str(&r.to_string());
        }

        summands.push(summand);
    }

    binary_sum(&summands, number_a.len() + max_len - 1)
}
