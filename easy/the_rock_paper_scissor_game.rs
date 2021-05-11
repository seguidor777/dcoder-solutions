use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: usize = input.trim().parse().expect("cannot parse input");

    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let mut items = input.chars();

    for _ in 0..n {
        let winner = match items.next().unwrap() {
            'R' => match items.next().unwrap() {
                'R' => "Draw",
                'P' => "You",
                'S' => "Dcoder",
                _ => ""
            },
            'P' => match items.next().unwrap() {
                'R' => "Dcoder",
                'P' => "Draw",
                'S' => "You",
                _ => ""
            },
            'S' => match items.next().unwrap() {
                'R' => "You",
                'P' => "Dcoder",
                'S' => "Draw",
                _ => ""
            },
            _ => ""
        };

        println!("{}", winner);
    }
}
