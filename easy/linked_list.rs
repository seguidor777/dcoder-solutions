use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let list: Vec<&str> = input.split_whitespace().collect();
    let rev_list = list[..list.len() - 1]
        .iter()
        .rev()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", rev_list);
}
