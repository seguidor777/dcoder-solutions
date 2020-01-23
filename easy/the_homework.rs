use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let t: u32 = input.trim().parse().expect("cannot parse input");

    // Constraints
    if t >= 1 && t <= 100 {
        'outer: for _ in 0..t {
            input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("cannot read input");

            let k: u32 = input.trim().parse().expect("cannot parse input");

            input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("cannot read input");

            let mut v: Vec<String> = input.split_whitespace().map(String::from).collect();

            // Constraints
            if k >= 1 && k <= 100 && k as usize == v.len() {
                for d in &v {
                    let parsed_digit: i32 = (*d).parse().unwrap();

                    if parsed_digit < 0 || parsed_digit > 9 {
                        continue 'outer;
                    }
                }

                // Find the largest number possible using the provided digits
                v.sort_by(|x, y| {
                    let xy = format!("{}{}", x, y);
                    let yx = format!("{}{}", y, x);
                    xy.cmp(&yx).reverse()
                });

                println!("{}", v.join(""));
            }
        }
    }
}
