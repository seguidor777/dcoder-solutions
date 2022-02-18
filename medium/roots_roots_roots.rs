use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let v: Vec<f32> = input
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();
    let (a, b, c) = (v[0], v[1], v[2]);

    match b * b - 4. * a * c >= 0. {
        true => println!("{:.2}", -b / a),
        false => println!(" "),
    }
}
