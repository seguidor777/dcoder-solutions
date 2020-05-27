use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let binary: Vec<i32> = input.split("").filter_map(|x| x.parse().ok()).collect();

    for bit in binary {
        print!("{}", bit ^ 1);
    }
}
