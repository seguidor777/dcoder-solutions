use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let versions: Vec<f32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let (v, x) = (versions[0], versions[1]);

    if x >= v {
        println!("Yes");
    } else {
        println!("No");
    }
}
