use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let t: u32 = input.trim().parse().expect("cannot parse input");

    // Constraints
    if t >= 1 && t <= 100 {
        let mut output: Vec<String> = vec![];

        for _ in 0..t {
            let mut s = String::new();

            io::stdin().read_line(&mut s).expect("cannot read input");

            if s.len() <= 1000 {
                output.push(s.to_ascii_uppercase());
            }
        }

        println!("{}", output.join(""))
    }
}
