use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: u32 = input.trim().parse().expect("cannot parse input");

    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let mut evens: Vec<i32> = vec![];

    // Constraints
    if n >= 1 && n <= 10000 {
        for (i, item) in v.iter().enumerate() {
            if *item < 1 || *item > 1000000 {
                return;
            }

            if (i + 1) % 2 == 0 && *item % 2 == 0 {
                evens.push(*item);
            }
        }

        println!(
            "{}",
            evens
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
