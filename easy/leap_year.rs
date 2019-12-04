use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("unable to read input");
    let year: u32 = input.trim().parse().expect("unable to trim input");

    // Constraints
    if year >= 1000 && year <= 100000 {
        // Leap year logic
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
