use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let scores: Vec<u32> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let mean = scores.iter().sum::<u32>() as f32 / scores.len() as f32;

    let grade = match mean {
        x if x > 90.0 => "A",
        x if x > 80.0 => "B",
        x if x > 70.0 => "C",
        x if x > 60.0 => "D",
        _ => "F",
    };

    print!("{}", grade);
}
