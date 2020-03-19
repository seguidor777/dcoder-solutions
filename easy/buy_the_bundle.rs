use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("cannot read input");

    let t: u32 = input.trim().parse().expect("cannot parse input");

    for _ in 0..t {
        input = String::new();
        io::stdin().read_line(&mut input).expect("cannot read input");

        let v: Vec<u32> = input.split_whitespace().filter_map(|x| x.parse().ok()).collect();
        let (n, m) = (v[0], v[1]);

        if m % n == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
