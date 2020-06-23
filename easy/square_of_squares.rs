use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: i32 = input.trim().parse().expect("cannot parse input");

    match (n as f32).sqrt() {
        x if x - x.floor() == 0.0 => println!("YES"),
        _ => println!("NO"),
    }
}
