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

    // Constraints
    if n >= 1 && n <= 100 {
        // Find the remainder when the sum is divided by the largest number
        // of the array
        let mut max: i32 = v[0];

        // Validate item's values and determine max
        for i in 0..v.len() {
            if v[i] >= 0 && v[i] <= 1000 {
                if v[i] > max {
                    max = v[i];
                };
            } else {
                break;
            }
        }

        let remainder = v.iter().sum::<i32>() % max;

        println!("{}", remainder);
    }
}
