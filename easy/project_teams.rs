use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let mut n: u32 = input.trim().parse().expect("cannot parse input");

    // Constraints
    if n >= 1 && n <= 100000 {
        // Calculate number of groups
        let mut result: u32 = 0;

        // Values from six can follow the next formula
        if n > 6 {
            let module: u32 = n % 3;

            n -= module * 4;
            result = module + n / 3;
        } else {
            result = n / 3;
        }

        println!("{}", result);
    }
}
