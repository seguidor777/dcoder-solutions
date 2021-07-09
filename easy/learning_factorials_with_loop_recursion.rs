use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: u32 = input.trim().parse().expect("cannot parse input");

    match n {
        0 | 1 => println!("1"),
        _ => {
            let factorial: i32 = (2..=n).fold(1, |acc, x| acc * x);

            println!("{}", factorial)
        }
    }
}
