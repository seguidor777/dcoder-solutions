use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let t: i32 = input.trim().parse().expect("cannot parse input");

    for _ in 0..t {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("cannot read input");

        let mut chars = input.trim().chars();
        let first = chars.nth(0).unwrap().to_digit(10).unwrap();
        let last = chars.last().unwrap().to_digit(10).unwrap();

        println!("{}", first + last);
    }
}
