use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("cannot read input");
    
    let half = &input[..input.len() / 2];

    println!("{}", half);
}
