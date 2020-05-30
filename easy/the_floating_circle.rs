use std::io;

const PI: f32 = 3.14;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let radio: f32 = input.trim().parse().expect("cannot parse input");

    if radio <= 0.0 && radio >= -1_000.0 {
        print!("0");
    } else if radio > 0.0 && radio <= 1_000.0 {
        let area = PI * radio.powi(2);

        print!("{:.2}", area);
    }
}
