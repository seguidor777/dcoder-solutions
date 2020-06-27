use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let vowels = ['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];
    let vowels_count = input
        .trim()
        .chars()
        .filter(|x| vowels.iter().any(|&v| v == *x))
        .count();

    println!("{}", vowels_count);
}
