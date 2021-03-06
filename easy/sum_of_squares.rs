use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("cannot read input");

    let v: Vec<i32> = input.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    let result = (v[0]..=v[1]).fold(0, |sum, num| sum + num.pow(2));
    
    println!("{}", result);
}
