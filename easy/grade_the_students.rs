use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("cannot read line");

    let t: u32 = input.trim().parse().expect("cannot parse input");

    // Constraints
    if t >= 1 && t <= 10 {
        for _ in 0..t {
            input = String::new();
            io::stdin().read_line(&mut input).expect("cannot read line");

            let v: Vec<i32> = input
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();

            // Constraints
            if v[0] < 1 || v[0] > 100 || v[1] < 1 || v[1] > 100 {
                continue;
            }

            if v[0] > 70 && v[1] > 50 {
                println!("Pass");
            } else {
                println!("Fail");
            }
        }
    }
}
