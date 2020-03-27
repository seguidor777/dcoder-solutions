use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: u32 = input.trim().parse().expect("cannot parse input");
    let v: Vec<u32> = input.split("").filter_map(|x| x.parse().ok()).collect();
    let sum = v.iter().map(|x| x.pow(3)).sum();

    if n == sum {
        println!("YES");
    } else {
        println!("NO");
    }
}
