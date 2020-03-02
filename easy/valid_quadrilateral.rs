use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let t: i32 = input.trim().parse().expect("cannot parse input");

    for _ in 0..t {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("cannot read input");

        let v: Vec<i32> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        let sum: i32 = v.iter().sum();

        if sum == 360 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
