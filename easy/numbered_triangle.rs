use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: u32 = input.trim().parse().expect("cannot parse input");

    // Constraints
    if n >= 1 && n <= 9 {
        for last in 1..=n {
            let row: Vec<String> = (1..=last).map(|x| x.to_string()).collect();

            println!("{}", row.join(" "));
        }
    }
}
