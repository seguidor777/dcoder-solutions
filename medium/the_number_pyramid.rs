use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("cannot read input");

    let n: usize = input.trim().parse().expect("cannot parse input");

    for level in (1..=n) {
        let row: String = (1..=level).rev().chain(2..=level).map(|d| std::char::from_digit(d as u32, 10).unwrap()).collect();
        let spaces = n - level;

        println!("{:1$}{2}", "", spaces, row);
    }
}
