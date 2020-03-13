use std::io;

fn main() {
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("cannot read input");
    n = n.trim().to_string();

    let last = n.chars().last().unwrap();
    let suffix: String;

    if last == '1' {
        suffix = "st".to_string();
    } else if last == '2' {
        suffix = "nd".to_string();
    } else if last == '3' {
        suffix = "rd".to_string();
    } else {
        suffix = "th".to_string();
    }

    println!("{}{}", n, suffix);
}
