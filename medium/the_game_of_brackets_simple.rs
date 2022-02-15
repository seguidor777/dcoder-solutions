use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    match is_matched(&input.trim()) {
        true => println!("Yes"),
        false => println!("No"),
    }
}

fn is_matched(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in input.chars() {
        if c == '(' {
            stack.push(')');
        } else if !stack.is_empty() && c == ')' {
            stack.pop();
        } else {
            return false
        }
    }

    stack.is_empty()
}
