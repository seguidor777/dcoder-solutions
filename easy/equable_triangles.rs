use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: u32 = input.trim().parse().expect("cannot parse input");

    for _ in 0..n {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("cannot read input");

        let sides: Vec<f32> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        let sides_sum = sides.iter().sum();
        let s = sides_sum as f32 / 2.0;
        let (a, b, c) = (sides[0], sides[1], sides[2]);
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();

        if area == sides_sum {
            println!("True");
        } else {
            println!("False");
        }
    }
}
