use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let celsius: i32 = input.trim().parse().expect("cannot parse input");

    // Constraints
    if celsius >= 0 && celsius <= 1000 {
        // Convert degree Celsius to Farenheit
        let farenheit = celsius as f32 * 1.8 + 32.0;

        println!("{}", farenheit);
    }
}
